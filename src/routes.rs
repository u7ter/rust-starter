use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::Config;
use crate::handlers;
use crate::handlers::auth_handler::{__path_login, __path_register};
use crate::handlers::health_handler::{__path_healthz, __path_ready};
use crate::middleware::{rate_limit_middleware, RateLimitLayer};
use crate::repositories::UserRepository;
use crate::services::AuthService;

#[derive(OpenApi)]
#[openapi(
    paths(
        healthz,
        ready,
        register,
        login,
    ),
    components(
        schemas(
            crate::models::RegisterRequest,
            crate::models::LoginRequest,
            crate::models::LoginResponse,
            crate::models::user::UserResponse,
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "health", description = "Health check endpoints")
    )
)]
pub struct ApiDoc;

pub fn create_routes(pool: PgPool, config: Config) -> Router {
    // Initialize repositories
    let user_repository = UserRepository::new(pool.clone());

    // Initialize services
    let auth_service = AuthService::new(
        user_repository,
        config.jwt_secret.clone(),
        config.jwt_expiration_hours,
    );

    // Initialize rate limiter
    let rate_limit_layer = RateLimitLayer::new(config.rate_limit_rps, config.rate_limit_burst);
    let limiter = rate_limit_layer.limiter();

    // Health check routes (no rate limiting)
    let health_routes = Router::new()
        .route("/healthz", get(handlers::healthz))
        .route("/ready", get(handlers::ready))
        .with_state(pool.clone());

    // Auth routes
    let auth_routes = Router::new()
        .route("/auth/register", post(handlers::register))
        .route("/auth/login", post(handlers::login))
        .with_state(auth_service.clone());

    // Combine routes
    let mut app = Router::new()
        .merge(health_routes)
        .merge(auth_routes)
        .layer(middleware::from_fn(move |req, next| {
            rate_limit_middleware(limiter.clone(), req, next)
        }));

    // Add Swagger UI in development mode
    if !config.is_production() {
        app =
            app.merge(SwaggerUi::new("/api-docs").url("/api-docs/openapi.json", ApiDoc::openapi()));
    }

    app
}
