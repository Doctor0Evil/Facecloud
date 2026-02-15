use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use std::sync::{Arc, Mutex};
use tracing::info;

use facecloud_core::neuromorphic::signals::InterfaceTelemetry;
use facecloud_core::safety::guard::{GuardKernel, GuardRecommendation};
use facecloud_core::safety::metrics::SafetyMetrics;
use facecloud_dna_auth::mfa::{evaluate_mfa, MultiLayerContext};
use facecloud_dna_auth::policy::{evaluate_policy, AccessPolicy};

#[derive(Clone)]
pub struct AppState {
    pub guard: GuardKernel,
    pub metrics: Arc<Mutex<SafetyMetrics>>,
}

pub fn app_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/evaluate/envelope", post(evaluate_envelope))
        .route("/evaluate/mfa", post(evaluate_mfa_route))
        .route("/metrics", get(metrics))
        .with_state(state)
}

async fn health() -> &'static str {
    "OK"
}

async fn evaluate_envelope(
    State(state): State<AppState>,
    Json(telemetry): Json<InterfaceTelemetry>,
) -> Json<GuardRecommendation> {
    let rec = state.guard.evaluate(&telemetry);
    {
        let metrics = state.metrics.lock().unwrap();
        metrics.observe_status(rec.evaluation.status, rec.evaluation.composite_margin);
    }
    info!("Envelope evaluation: {:?}", rec.message);
    Json(rec)
}

async fn evaluate_mfa_route(
    Json(ctx): Json<MultiLayerContext>,
) -> Json<(facecloud_dna_auth::mfa::AuthEvaluation, AccessPolicy)> {
    let auth_eval = evaluate_mfa(&ctx);
    let policy = AccessPolicy::default();
    Json((auth_eval, policy))
}

async fn metrics(State(state): State<AppState>) -> String {
    let metrics = state.metrics.lock().unwrap();
    metrics.export_prometheus()
}
