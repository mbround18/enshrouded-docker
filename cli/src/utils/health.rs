use std::net::UdpSocket;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use tracing::{debug, info, warn};

/// Target used for every health-check log line, so operators can isolate this
/// stream from the rest of the server/monitor logs, e.g. `RUST_LOG=health=debug`.
const LOG_TARGET: &str = "health";

/// How often the background poller refreshes [`HealthState`].
const POLL_INTERVAL: Duration = Duration::from_secs(5);

/// Snapshot of what we know about the game server, refreshed on an interval by
/// [`run_health_poller`] and read (lock-free) by the HTTP probe handlers.
///
/// Probes must never do work per request -- Kubernetes hits them every few
/// seconds and a probe that shells out or scans `/proc` inline would both add
/// latency and change what it measures under load. So polling happens once,
/// centrally, and every reader sees the last snapshot.
pub struct HealthState {
    /// The `enshrouded_server.exe` process exists.
    process_up: AtomicBool,
    /// Something is bound to the game port, i.e. players can connect.
    port_bound: AtomicBool,
    started_at: Instant,
    game_port: u16,
    query_port: u16,
}

impl HealthState {
    pub fn new(game_port: u16, query_port: u16) -> Self {
        Self {
            process_up: AtomicBool::new(false),
            port_bound: AtomicBool::new(false),
            started_at: Instant::now(),
            game_port,
            query_port,
        }
    }

    /// Liveness: the server process is up. A false here means the container
    /// should be restarted -- the process died and isn't coming back on its
    /// own.
    pub fn is_alive(&self) -> bool {
        self.process_up.load(Ordering::Relaxed)
    }

    /// Readiness: the game port is accepting connections, so it's worth
    /// sending players here. False during the (long) startup and world-load
    /// phase, and again across a restart.
    pub fn is_ready(&self) -> bool {
        self.port_bound.load(Ordering::Relaxed)
    }

    pub fn uptime_seconds(&self) -> u64 {
        self.started_at.elapsed().as_secs()
    }

    pub const fn game_port(&self) -> u16 {
        self.game_port
    }

    pub const fn query_port(&self) -> u16 {
        self.query_port
    }
}

/// Best-effort guess at the address players should use to reach this server.
///
/// `PUBLIC_IP` wins if set (needed behind NAT/port-forwarding, where the
/// container/host address isn't what's actually reachable). Otherwise we ask
/// the kernel which local address it would route a packet to the internet
/// through -- this opens no socket to the network, it's a local routing-table
/// lookup, but it still only reflects this host's address, not anything
/// upstream of a NAT.
fn detect_host_ip() -> String {
    if let Ok(ip) = std::env::var("PUBLIC_IP") {
        let ip = ip.trim();
        if !ip.is_empty() {
            return ip.to_string();
        }
    }

    UdpSocket::bind("0.0.0.0:0")
        .and_then(|socket| {
            socket.connect("8.8.8.8:80")?;
            socket.local_addr()
        })
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|_| "<your-server-ip>".to_string())
}

/// Checks whether something is bound to `port` (UDP) on this host by trying
/// to bind it ourselves. Enshrouded's game protocol doesn't offer a
/// lightweight "are you alive" query packet, but a bound socket is a
/// reliable proxy for "the process is up and the game port is open" -- which
/// is what a player's client actually needs to connect.
///
/// This used to parse `/proc/net/udp{,6}` for a matching local port, but that
/// approach produced false negatives in practice (e.g. players could join
/// while the check kept reporting the port as free). Attempting a real
/// `bind()` is what the kernel itself uses to decide "is this port in use",
/// so it can't drift out of sync with reality the way text-table parsing
/// can: success means the port is free (nothing bound), and failure --
/// specifically `EADDRINUSE`/`EADDRNOTAVAIL` -- means something already
/// holds it.
fn port_bound(port: u16) -> bool {
    // A bind error only means "in use" if it's actually AddrInUse -- other
    // errors (e.g. IPv6 disabled on this host) aren't evidence of anything
    // and shouldn't be read as the game holding the port.
    let in_use = |result: std::io::Result<UdpSocket>| {
        matches!(
            result,
            Err(e) if e.kind() == std::io::ErrorKind::AddrInUse
        )
    };

    in_use(UdpSocket::bind(("0.0.0.0", port))) || in_use(UdpSocket::bind(("::", port)))
}

/// Whether a process whose command line mentions `needle` is running.
///
/// Walks `/proc` directly rather than pulling in a process-listing crate:
/// `gsm-instance`'s own `ServerProcess` isn't re-exported, and the one thing
/// we need here -- "does any process mention enshrouded_server.exe" -- is a
/// handful of lines. The game runs under Proton, so the match has to be
/// against the *command line* (the Linux-visible process name is Proton's
/// wrapper, not the Windows executable).
fn process_matching(needle: &str) -> bool {
    let Ok(entries) = std::fs::read_dir("/proc") else {
        return false;
    };

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        // /proc contains plenty of non-pid entries; a non-numeric name is one.
        if !path
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.bytes().all(|b| b.is_ascii_digit()))
        {
            continue;
        }

        if cmdline_contains(&path.join("cmdline"), needle) {
            return true;
        }
    }

    false
}

fn cmdline_contains(cmdline: &Path, needle: &str) -> bool {
    // cmdline is NUL-separated; replacing the separators keeps argv boundaries
    // from hiding a match that spans them.
    std::fs::read(cmdline).is_ok_and(|raw| {
        String::from_utf8_lossy(&raw)
            .replace('\0', " ")
            .to_ascii_lowercase()
            .contains(&needle.to_ascii_lowercase())
    })
}

/// Polls the server on an interval, updating `state` and logging transitions.
///
/// Every check is logged at debug (own target, so it doesn't spam the normal
/// info-level output). The moment the port is first seen accepting
/// connections, an info-level line is logged with a `steam://connect` URL the
/// user can hand straight to their Steam client to join.
pub async fn run_health_poller(state: Arc<HealthState>) {
    let host = detect_host_ip();
    let game_port = state.game_port();
    let mut interval = tokio::time::interval(POLL_INTERVAL);

    loop {
        interval.tick().await;

        let was_ready = state.is_ready();
        let alive = port_bound(game_port);
        state.port_bound.store(alive, Ordering::Relaxed);
        state
            .process_up
            .store(process_matching("enshrouded_server.exe"), Ordering::Relaxed);

        match (alive, was_ready) {
            (true, false) => {
                info!(
                    target: LOG_TARGET,
                    "🎮 Game is accepting connections! Launch it with: steam://connect/{host}:{game_port}"
                );
            }
            (false, true) => {
                warn!(
                    target: LOG_TARGET,
                    "Game port {game_port} is no longer accepting connections"
                );
            }
            (true, true) => {
                debug!(target: LOG_TARGET, "Health check: game port {game_port} still accepting connections");
            }
            (false, false) => {
                debug!(target: LOG_TARGET, "Health check: game port {game_port} not yet accepting connections");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn port_bound_reports_a_socket_we_hold() {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("bind an ephemeral port");
        let port = socket.local_addr().expect("local addr").port();
        assert!(port_bound(port));
        drop(socket);
        assert!(!port_bound(port));
    }

    #[test]
    fn process_matching_finds_this_test_binary_and_not_nonsense() {
        // Our own cmdline is in /proc, so a substring of it must match.
        assert!(process_matching("enshrouded"));
        assert!(!process_matching("definitely-not-a-running-process-xyzzy"));
    }

    #[test]
    fn health_state_starts_down() {
        let state = HealthState::new(15636, 15637);
        assert!(!state.is_alive());
        assert!(!state.is_ready());
        assert_eq!(state.game_port(), 15636);
        assert_eq!(state.query_port(), 15637);
    }
}
