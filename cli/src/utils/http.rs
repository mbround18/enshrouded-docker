//! A deliberately tiny HTTP surface for container orchestrators.
//!
//! Kubernetes (and Docker's own `HEALTHCHECK`) need a probe they can call;
//! Enshrouded's game ports can't serve that purpose because they speak a game
//! protocol with no plain "are you up" query, and TCP-connecting to them tells
//! you nothing useful (#55). The monitor process already tracks whether the
//! server is up and reachable, so it exposes that over HTTP rather than making
//! every caller re-derive it.
//!
//! | Route                     | 200 when                                   |
//! | ------------------------- | ------------------------------------------ |
//! | `/live`, `/liveness`      | the `enshrouded_server.exe` process is up  |
//! | `/ready`, `/readiness`    | the game port is accepting connections     |
//! | `/health`                 | both of the above                          |
//!
//! Everything else returns `503` with a JSON body naming the reason, which is
//! what a probe wants: no body parsing needed for the decision, detail
//! available when a human goes looking.

use crate::utils::health::HealthState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use axum::{Router, routing::get};
use serde_json::json;
use std::sync::Arc;
use tracing::{error, info};

/// Default port for the probe endpoints. Matches the sibling `valheim-docker`
/// project so the two containers can be probed the same way.
const DEFAULT_HTTP_PORT: u16 = 3000;

/// The port the probe server listens on, or `None` when disabled.
///
/// `HTTP_PORT=0` turns the server off for anyone who would rather not have an
/// extra listener in the container.
pub fn configured_port() -> Option<u16> {
    let raw = gsm_shared::fetch_var("HTTP_PORT", &DEFAULT_HTTP_PORT.to_string());
    match raw.trim().parse::<u16>() {
        Ok(0) => None,
        Ok(port) => Some(port),
        Err(e) => {
            error!("Invalid HTTP_PORT='{raw}' ({e}). Falling back to {DEFAULT_HTTP_PORT}.");
            Some(DEFAULT_HTTP_PORT)
        }
    }
}

fn status_for(ok: bool) -> StatusCode {
    if ok {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}

async fn liveness(State(state): State<Arc<HealthState>>) -> impl IntoResponse {
    let alive = state.is_alive();
    (
        status_for(alive),
        Json(json!({
            "alive": alive,
            "status": if alive { "running" } else { "starting" },
            "uptimeSeconds": state.uptime_seconds(),
        })),
    )
}

async fn readiness(State(state): State<Arc<HealthState>>) -> impl IntoResponse {
    let ready = state.is_ready();
    (
        status_for(ready),
        Json(json!({
            "ready": ready,
            "status": if ready { "accepting connections" } else { "not accepting connections" },
            "gamePort": state.game_port(),
            "uptimeSeconds": state.uptime_seconds(),
        })),
    )
}

async fn health(State(state): State<Arc<HealthState>>) -> impl IntoResponse {
    let alive = state.is_alive();
    let ready = state.is_ready();
    (
        status_for(alive && ready),
        Json(json!({
            "alive": alive,
            "ready": ready,
            "gamePort": state.game_port(),
            "queryPort": state.query_port(),
            "uptimeSeconds": state.uptime_seconds(),
        })),
    )
}

pub fn router(state: Arc<HealthState>) -> Router {
    Router::new()
        .route("/live", get(liveness))
        .route("/liveness", get(liveness))
        .route("/ready", get(readiness))
        .route("/readiness", get(readiness))
        .route("/health", get(health))
        .with_state(state)
}

/// Serves the probe endpoints until the process exits.
///
/// A bind failure is logged, not fatal: losing the probes shouldn't take down
/// a game server that is otherwise running fine.
pub async fn serve(state: Arc<HealthState>, port: u16) {
    let listener = match tokio::net::TcpListener::bind(("0.0.0.0", port)).await {
        Ok(listener) => listener,
        Err(e) => {
            error!("Failed to bind the health endpoint on port {port}: {e}");
            return;
        }
    };

    info!("🩺 Health endpoints on http://0.0.0.0:{port} (/live, /ready, /health)");
    if let Err(e) = axum::serve(listener, router(state)).await {
        error!("Health endpoint server stopped: {e}");
    }
}

/// Queries this container's own probe endpoint and reports whether it is
/// healthy.
///
/// Exists so the image's `HEALTHCHECK` (and anyone with a shell in the
/// container) can check the server without `curl` or `wget`, neither of which
/// the base image ships.
pub fn probe_locally(route: &str) -> Result<bool, String> {
    let port = configured_port().ok_or_else(|| {
        "Health endpoints are disabled (HTTP_PORT=0); nothing to probe".to_string()
    })?;
    let url = format!("http://127.0.0.1:{port}/{route}");

    let response = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?
        .get(&url)
        .send()
        .map_err(|e| format!("{url} is not answering: {e}"))?;

    let healthy = response.status().is_success();
    let status = response.status();
    let body = response.text().unwrap_or_default();
    info!("{url} -> {status} {body}");
    Ok(healthy)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use axum::http::Request;
    use tower::ServiceExt;

    async fn get_route(state: Arc<HealthState>, path: &str) -> (StatusCode, serde_json::Value) {
        let response = router(state)
            .oneshot(
                Request::builder()
                    .uri(path)
                    .body(axum::body::Body::empty())
                    .expect("build request"),
            )
            .await
            .expect("route responds");
        let status = response.status();
        let bytes = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("read body");
        (status, serde_json::from_slice(&bytes).expect("json body"))
    }

    #[tokio::test]
    async fn probes_report_503_while_the_server_is_down() {
        let state = Arc::new(HealthState::new(15636, 15637));
        for path in ["/live", "/liveness", "/ready", "/readiness", "/health"] {
            let (status, _) = get_route(Arc::clone(&state), path).await;
            assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE, "{path}");
        }
    }

    #[tokio::test]
    async fn readiness_body_names_the_game_port() {
        let state = Arc::new(HealthState::new(25000, 25001));
        let (_, body) = get_route(state, "/ready").await;
        assert_eq!(body["gamePort"], 25000);
        assert_eq!(body["ready"], false);
    }

    /// Exercises the real `serve` + `probe_locally` path, i.e. what the
    /// image's HEALTHCHECK actually runs, rather than only the router.
    #[tokio::test(flavor = "multi_thread")]
    async fn probe_locally_talks_to_the_served_endpoint() {
        // Pick a free port, then let `serve` rebind it.
        let port = {
            let probe = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
            probe.local_addr().expect("addr").port()
        };

        let state = Arc::new(HealthState::new(15636, 15637));
        tokio::spawn(serve(Arc::clone(&state), port));

        // `probe_locally` is blocking, so it can't run on this runtime's thread.
        let healthy = tokio::task::spawn_blocking(move || {
            // Serialized against the other HTTP_PORT test by the port being
            // read once, up front, inside the blocking task.
            unsafe { std::env::set_var("HTTP_PORT", port.to_string()) };
            for _ in 0..50 {
                if let Ok(result) = probe_locally("health") {
                    return Some(result);
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            None
        })
        .await
        .expect("blocking task");

        unsafe { std::env::remove_var("HTTP_PORT") };

        // The server is down in this test, so the probe must report unhealthy
        // -- but it must have gotten an answer, which is the point.
        assert_eq!(healthy, Some(false));
    }

    #[test]
    fn configured_port_honours_the_env_var() {
        unsafe { std::env::set_var("HTTP_PORT", "9090") };
        assert_eq!(configured_port(), Some(9090));
        unsafe { std::env::set_var("HTTP_PORT", "0") };
        assert_eq!(configured_port(), None);
        unsafe { std::env::remove_var("HTTP_PORT") };
        assert_eq!(configured_port(), Some(DEFAULT_HTTP_PORT));
    }
}
