use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub server_port: u16,
    pub server_host: String,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
    pub rate_limit_rps: u32,
    pub rate_limit_burst: u32,
    pub environment: Environment,
    pub allowed_origins: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Environment {
    Development,
    Production,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        dotenvy::dotenv().ok();

        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .map_err(|_| "Invalid SERVER_PORT")?;

        let server_host = env::var("SERVER_HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL must be set")?;

        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| "JWT_SECRET must be set")?;

        let jwt_expiration_hours = env::var("JWT_EXPIRATION_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse()
            .map_err(|_| "Invalid JWT_EXPIRATION_HOURS")?;

        let rate_limit_rps = env::var("RATE_LIMIT_RPS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .map_err(|_| "Invalid RATE_LIMIT_RPS")?;

        let rate_limit_burst = env::var("RATE_LIMIT_BURST")
            .unwrap_or_else(|_| "20".to_string())
            .parse()
            .map_err(|_| "Invalid RATE_LIMIT_BURST")?;

        let environment = match env::var("ENV")
            .unwrap_or_else(|_| "development".to_string())
            .to_lowercase()
            .as_str()
        {
            "production" => Environment::Production,
            _ => Environment::Development,
        };

        let allowed_origins = env::var("ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:3000".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        Ok(Config {
            server_port,
            server_host,
            database_url,
            jwt_secret,
            jwt_expiration_hours,
            rate_limit_rps,
            rate_limit_burst,
            environment,
            allowed_origins,
        })
    }

    pub fn is_production(&self) -> bool {
        self.environment == Environment::Production
    }
}
