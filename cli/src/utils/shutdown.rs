use nix::sys::signal::{Signal, kill};
use nix::unistd::Pid;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};
use tracing::{debug, info, warn};

const SERVER_EXECUTABLE: &str = "enshrouded_server.exe";

/// Stop the running Enshrouded server and wait for it to exit.
///
/// The server runs under Proton, so the pid we spawn is Proton's launcher,
/// not the game, and no pid file is written (see `setup::spawn_server`). We
/// find the game by its exact executable name instead of relying on
/// `Instance::stop()`: that either does nothing without a pid file, or (in
/// older gsm-instance releases) fuzzy-matches process names, which also
/// matches this `enshrouded` CLI and then waits on it forever.
///
/// The game gets SIGINT so it can save, `STOP_TIMEOUT` seconds (default 90)
/// to exit, and SIGKILL after that, so a restart can always proceed.
pub fn stop_server() -> Result<(), Box<dyn std::error::Error>> {
    let timeout = std::env::var("STOP_TIMEOUT")
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .unwrap_or(90);
    stop_processes(
        Path::new("/proc"),
        SERVER_EXECUTABLE,
        Duration::from_secs(timeout),
        Duration::from_secs(1),
    )
}

fn stop_processes(
    proc_root: &Path,
    executable: &str,
    timeout: Duration,
    poll: Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    let pids = find_pids(proc_root, executable);
    if pids.is_empty() {
        info!(
            "No {} process found; server is already stopped.",
            executable
        );
        return Ok(());
    }

    for pid in &pids {
        info!("Sending interrupt to {} (pid {})", executable, pid);
        signal(*pid, Signal::SIGINT);
    }

    let deadline = Instant::now() + timeout;
    loop {
        let remaining = find_pids(proc_root, executable);
        if remaining.is_empty() {
            info!("Server stopped.");
            return Ok(());
        }
        if Instant::now() >= deadline {
            for pid in &remaining {
                warn!(
                    "{} (pid {}) still running after {:?}; killing it",
                    executable, pid, timeout
                );
                signal(*pid, Signal::SIGKILL);
            }
            thread::sleep(poll);
            if find_pids(proc_root, executable).is_empty() {
                return Ok(());
            }
            return Err(format!("{} did not exit after SIGKILL", executable).into());
        }
        debug!("Waiting for {} to exit: {:?}", executable, remaining);
        thread::sleep(poll);
    }
}

fn signal(pid: i32, signal: Signal) {
    // ESRCH just means it exited between the scan and the signal.
    if let Err(e) = kill(Pid::from_raw(pid), signal) {
        debug!("Could not send {:?} to pid {}: {}", signal, pid, e);
    }
}

/// Pids under `proc_root` whose argv[0] is `executable`, ignoring this process.
fn find_pids(proc_root: &Path, executable: &str) -> Vec<i32> {
    let own_pid = std::process::id() as i32;
    let Ok(entries) = fs::read_dir(proc_root) else {
        return Vec::new();
    };

    let mut pids: Vec<i32> = entries
        .filter_map(|entry| entry.ok()?.file_name().to_str()?.parse::<i32>().ok())
        .filter(|pid| *pid != own_pid)
        .filter(|pid| {
            fs::read(proc_root.join(pid.to_string()).join("cmdline"))
                .is_ok_and(|cmdline| is_executable(&cmdline, executable))
        })
        .collect();
    pids.sort_unstable();
    pids
}

/// Whether a NUL-separated `/proc/<pid>/cmdline` was started as `executable`.
///
/// Only argv[0] is compared, so Proton's launcher (`python3 proton run
/// .../enshrouded_server.exe`) doesn't match. Wine reports argv[0] as a
/// Windows path, so both `/` and `\` count as separators. Zombies have an
/// empty cmdline and never match.
fn is_executable(cmdline: &[u8], executable: &str) -> bool {
    let argv0 = cmdline.split(|b| *b == 0).next().unwrap_or_default();
    let argv0 = String::from_utf8_lossy(argv0);
    argv0
        .rsplit(['/', '\\'])
        .next()
        .is_some_and(|name| name.eq_ignore_ascii_case(executable))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::process::CommandExt;
    use std::process::Command;
    use tempfile::tempdir;

    #[test]
    fn matches_unix_and_windows_paths() {
        assert!(is_executable(
            b"/home/steam/enshrouded/enshrouded_server.exe\0",
            SERVER_EXECUTABLE
        ));
        assert!(is_executable(
            b"Z:\\home\\steam\\enshrouded\\enshrouded_server.exe\0-log\0",
            SERVER_EXECUTABLE
        ));
        assert!(is_executable(b"enshrouded_server.exe", SERVER_EXECUTABLE));
    }

    #[test]
    fn ignores_cli_launcher_and_zombies() {
        assert!(!is_executable(b"enshrouded\0cron\0", SERVER_EXECUTABLE));
        assert!(!is_executable(
            b"python3\0/opt/proton/proton\0run\0/home/steam/enshrouded/enshrouded_server.exe\0",
            SERVER_EXECUTABLE
        ));
        assert!(!is_executable(b"", SERVER_EXECUTABLE));
    }

    #[test]
    fn find_pids_reads_proc_layout() {
        let proc_root = tempdir().unwrap();
        for (pid, cmdline) in [
            ("10", &b"Z:\\enshrouded_server.exe\0"[..]),
            ("11", b"enshrouded\0cron\0"),
            ("12", b"/x/enshrouded_server.exe\0"),
        ] {
            let dir = proc_root.path().join(pid);
            fs::create_dir(&dir).unwrap();
            fs::write(dir.join("cmdline"), cmdline).unwrap();
        }
        fs::create_dir(proc_root.path().join("self")).unwrap();

        assert_eq!(find_pids(proc_root.path(), SERVER_EXECUTABLE), vec![10, 12]);
    }

    #[test]
    fn stop_with_nothing_running_is_ok() {
        let proc_root = tempdir().unwrap();
        stop_processes(
            proc_root.path(),
            SERVER_EXECUTABLE,
            Duration::from_secs(1),
            Duration::from_millis(10),
        )
        .unwrap();
    }

    #[test]
    fn interrupt_stops_cooperative_process() {
        let name = "gsm-test-stop-interrupt";
        let mut child = Command::new("sleep").arg0(name).arg("30").spawn().unwrap();
        // Reap in the background so the exited child doesn't linger as a zombie.
        let reaper = thread::spawn(move || child.wait());

        stop_processes(
            Path::new("/proc"),
            name,
            Duration::from_secs(10),
            Duration::from_millis(50),
        )
        .unwrap();
        assert!(!reaper.join().unwrap().unwrap().success());
    }

    #[test]
    fn kills_process_that_ignores_interrupt() {
        let name = "gsm-test-stop-kill";
        let mut child = Command::new("bash")
            .args(["-c", &format!("trap '' INT; exec -a {name} sleep 30")])
            .spawn()
            .unwrap();
        // Wait until bash has exec'd into the renamed sleep.
        let deadline = Instant::now() + Duration::from_secs(5);
        while find_pids(Path::new("/proc"), name).is_empty() {
            assert!(Instant::now() < deadline, "test process never started");
            thread::sleep(Duration::from_millis(10));
        }
        let reaper = thread::spawn(move || child.wait());

        let started = Instant::now();
        stop_processes(
            Path::new("/proc"),
            name,
            Duration::from_millis(300),
            Duration::from_millis(50),
        )
        .unwrap();
        assert!(started.elapsed() >= Duration::from_millis(300));
        assert!(!reaper.join().unwrap().unwrap().success());
    }
}
