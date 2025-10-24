use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::models::{LoginRequest, RegisterRequest};
use crate::services::AuthService;

/// Register a new user
#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = LoginResponse),
        (status = 400, description = "Invalid request"),
        (status = 409, description = "User already exists")
    ),
    tag = "auth"
)]
pub async fn register(
    State(auth_service): State<AuthService>,
    Json(request): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AuthHandlerError> {
    let response = auth_service.register(request).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

/// Login with existing credentials
#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "auth"
)]
pub async fn login(
    State(auth_service): State<AuthService>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthHandlerError> {
    let response = auth_service.login(request).await?;
    Ok(Json(response))
}

// Error handling
#[derive(Debug)]
pub struct AuthHandlerError(crate::services::auth_service::AuthError);

impl From<crate::services::auth_service::AuthError> for AuthHandlerError {
    fn from(error: crate::services::auth_service::AuthError) -> Self {
        AuthHandlerError(error)
    }
}

impl IntoResponse for AuthHandlerError {
    fn into_response(self) -> axum::response::Response {
        use crate::services::auth_service::AuthError;

        let (status, message) = match self.0 {
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            AuthError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AuthError::PasswordHashError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Password hashing error")
            }
            AuthError::JwtError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "JWT error"),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
