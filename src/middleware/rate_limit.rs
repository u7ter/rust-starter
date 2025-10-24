use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use governor::{
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;

pub type SharedRateLimiter = Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>;

#[derive(Clone)]
pub struct RateLimitLayer {
    limiter: SharedRateLimiter,
}

impl RateLimitLayer {
    pub fn new(_requests_per_second: u32, burst_size: u32) -> Self {
        let quota = Quota::with_period(Duration::from_secs(1))
            .unwrap()
            .allow_burst(std::num::NonZeroU32::new(burst_size).unwrap());

        let limiter = Arc::new(RateLimiter::direct(quota));

        Self { limiter }
    }

    pub fn limiter(&self) -> SharedRateLimiter {
        self.limiter.clone()
    }
}

pub async fn rate_limit_middleware(
    limiter: SharedRateLimiter,
    request: Request,
    next: Next,
) -> Result<Response, RateLimitError> {
    match limiter.check() {
        Ok(_) => Ok(next.run(request).await),
        Err(_) => Err(RateLimitError),
    }
}

#[derive(Debug)]
pub struct RateLimitError;

impl IntoResponse for RateLimitError {
    fn into_response(self) -> Response {
        (
            StatusCode::TOO_MANY_REQUESTS,
            Json(json!({ "error": "Rate limit exceeded" })),
        )
            .into_response()
    }
}
