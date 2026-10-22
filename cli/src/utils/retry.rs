use std::fmt::Display;
use std::thread;
use std::time::Duration;
use tracing::warn;

/// Run a steamcmd install or update, retrying when it fails.
///
/// steamcmd's first run against a fresh Steam directory regularly fails with
/// `ERROR! Failed to install app '2278520' (Missing configuration)`: it has
/// just bootstrapped itself and doesn't have the app's configuration yet. The
/// same command succeeds when run again (#64), so we retry instead of starting
/// a server with no game files. `STEAMCMD_ATTEMPTS` sets the total number of
/// tries (default 3).
pub fn steamcmd<T, E: Display>(label: &str, op: impl FnMut() -> Result<T, E>) -> Result<T, E> {
    let attempts = std::env::var("STEAMCMD_ATTEMPTS")
        .ok()
        .and_then(|value| value.trim().parse::<u32>().ok())
        .filter(|attempts| *attempts > 0)
        .unwrap_or(3);
    retry(label, attempts, Duration::from_secs(5), op)
}

fn retry<T, E: Display>(
    label: &str,
    attempts: u32,
    delay: Duration,
    mut op: impl FnMut() -> Result<T, E>,
) -> Result<T, E> {
    let mut attempt = 1;
    loop {
        match op() {
            Ok(value) => return Ok(value),
            Err(e) if attempt < attempts => {
                warn!(
                    "{} failed (attempt {}/{}): {}; retrying in {:?}",
                    label, attempt, attempts, e, delay
                );
                thread::sleep(delay);
                attempt += 1;
            }
            Err(e) => return Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_first_success_without_retrying() {
        let mut calls = 0;
        let result: Result<u32, String> = retry("op", 3, Duration::ZERO, || {
            calls += 1;
            Ok(7)
        });
        assert_eq!(result, Ok(7));
        assert_eq!(calls, 1);
    }

    #[test]
    fn retries_until_success() {
        let mut calls = 0;
        let result: Result<u32, String> = retry("op", 3, Duration::ZERO, || {
            calls += 1;
            if calls < 2 {
                Err("Missing configuration".to_owned())
            } else {
                Ok(calls)
            }
        });
        assert_eq!(result, Ok(2));
        assert_eq!(calls, 2);
    }

    #[test]
    fn gives_up_after_the_last_attempt() {
        let mut calls = 0;
        let result: Result<(), String> = retry("op", 3, Duration::ZERO, || {
            calls += 1;
            Err(format!("failure {calls}"))
        });
        assert_eq!(result, Err("failure 3".to_owned()));
        assert_eq!(calls, 3);
    }
}
