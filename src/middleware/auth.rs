use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::models::Claims;
use crate::services::AuthService;

pub async fn auth_middleware(
    State(auth_service): State<AuthService>,
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AuthError::MissingToken)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AuthError::InvalidToken)?;

    let claims = auth_service
        .verify_token(token)
        .map_err(|_| AuthError::InvalidToken)?;

    // Insert claims into request extensions so handlers can access them
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authorization token"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid authorization token"),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

// Extension trait to easily get claims from request
pub trait ClaimsExt {
    fn claims(&self) -> Option<&Claims>;
}

impl ClaimsExt for Request {
    fn claims(&self) -> Option<&Claims> {
        self.extensions().get::<Claims>()
    }
}
