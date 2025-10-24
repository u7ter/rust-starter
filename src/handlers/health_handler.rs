use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};
use sqlx::PgPool;

/// Health check endpoint - verifies database connectivity
#[utoipa::path(
    get,
    path = "/healthz",
    responses(
        (status = 200, description = "Service is healthy", body = Value),
        (status = 503, description = "Service is unhealthy", body = Value)
    ),
    tag = "health"
)]
pub async fn healthz(State(pool): State<PgPool>) -> Result<Json<Value>, StatusCode> {
    // Check database connection
    match sqlx::query("SELECT 1").fetch_one(&pool).await {
        Ok(_) => Ok(Json(json!({
            "status": "healthy",
            "database": "connected"
        }))),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

/// Readiness check endpoint
#[utoipa::path(
    get,
    path = "/ready",
    responses(
        (status = 200, description = "Service is ready", body = Value)
    ),
    tag = "health"
)]
pub async fn ready() -> Json<Value> {
    Json(json!({
        "status": "ready"
    }))
}
