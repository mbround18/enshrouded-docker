use std::net::UdpSocket;
use std::time::Duration;
use tracing::{debug, info, warn};

/// Target used for every health-check log line, so operators can isolate this
/// stream from the rest of the server/monitor logs, e.g. `RUST_LOG=health=debug`.
const LOG_TARGET: &str = "health";

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

/// Checks whether something is bound to `port` (UDP) on this host by reading
/// `/proc/net/udp{,6}` instead of attempting a real handshake. Enshrouded's
/// game protocol doesn't offer a lightweight "are you alive" query packet, but
/// a bound socket is a reliable proxy for "the process is up and the game
/// port is open" -- which is what a player's client actually needs to connect.
fn port_bound(port: u16) -> bool {
    let hex_port = format!("{:04X}", port);

    for path in ["/proc/net/udp", "/proc/net/udp6"] {
        let Ok(contents) = std::fs::read_to_string(path) else {
            continue;
        };
        for line in contents.lines().skip(1) {
            let Some(local_address) = line.split_whitespace().nth(1) else {
                continue;
            };
            if let Some((_, port_hex)) = local_address.split_once(':') {
                if port_hex.eq_ignore_ascii_case(&hex_port) {
                    return true;
                }
            }
        }
    }

    false
}

/// Polls the game port on an interval and logs liveness transitions.
///
/// Every check is logged at debug (own target, so it doesn't spam the normal
/// info-level output). The moment the port is first seen accepting
/// connections, an info-level line is logged with a `steam://connect` URL the
/// user can hand straight to their Steam client to join.
pub async fn run_liveness_check(game_port: u16) {
    let host = detect_host_ip();
    let mut accepting = false;
    let mut interval = tokio::time::interval(Duration::from_secs(5));

    loop {
        interval.tick().await;
        let alive = port_bound(game_port);

        match (alive, accepting) {
            (true, false) => {
                accepting = true;
                info!(
                    target: LOG_TARGET,
                    "🎮 Game is accepting connections! Launch it with: steam://connect/{host}:{game_port}"
                );
            }
            (false, true) => {
                accepting = false;
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
