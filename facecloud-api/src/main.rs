mod config;
mod routes;

use crate::config::ApiConfig;
use crate::routes::{app_router, AppState};
use axum::Server;
use facecloud_core::neuromorphic::envelope::EnvelopeConfig;
use facecloud_core::safety::guard::GuardKernel;
use facecloud_core::safety::metrics::SafetyMetrics;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cfg = ApiConfig::default();
    let guard = GuardKernel {
        config: EnvelopeConfig::default(),
    };
    let metrics = SafetyMetrics::new();

    let state = AppState { guard, metrics };

    let app = app_router(state);
    let addr: SocketAddr = cfg.bind_addr.parse().expect("invalid bind address");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server failed");
}
