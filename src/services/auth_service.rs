use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use thiserror::Error;

use crate::models::{Claims, LoginRequest, LoginResponse, RegisterRequest, User};
use crate::repositories::UserRepository;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Password hashing error")]
    PasswordHashError,
    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
}

#[derive(Clone)]
pub struct AuthService {
    user_repository: UserRepository,
    jwt_secret: String,
    jwt_expiration_hours: i64,
}

impl AuthService {
    pub fn new(
        user_repository: UserRepository,
        jwt_secret: String,
        jwt_expiration_hours: i64,
    ) -> Self {
        Self {
            user_repository,
            jwt_secret,
            jwt_expiration_hours,
        }
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<LoginResponse, AuthError> {
        // Check if user already exists
        if let Some(_) = self.user_repository.find_by_email(&request.email).await? {
            return Err(AuthError::UserAlreadyExists);
        }

        // Hash password
        let password_hash = self.hash_password(&request.password)?;

        // Create user
        let user = self
            .user_repository
            .create(&request.email, &password_hash)
            .await?;

        // Generate JWT token
        let token = self.generate_token(&user)?;

        Ok(LoginResponse {
            token,
            user: user.into(),
        })
    }

    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse, AuthError> {
        // Find user by email
        let user = self
            .user_repository
            .find_by_email(&request.email)
            .await?
            .ok_or(AuthError::InvalidCredentials)?;

        // Verify password
        self.verify_password(&request.password, &user.password_hash)?;

        // Generate JWT token
        let token = self.generate_token(&user)?;

        Ok(LoginResponse {
            token,
            user: user.into(),
        })
    }

    #[allow(dead_code)]
    pub fn verify_token(&self, token: &str) -> Result<Claims, AuthError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }

    fn hash_password(&self, password: &str) -> Result<String, AuthError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AuthError::PasswordHashError)?
            .to_string();

        Ok(password_hash)
    }

    fn verify_password(&self, password: &str, password_hash: &str) -> Result<(), AuthError> {
        let parsed_hash =
            PasswordHash::new(password_hash).map_err(|_| AuthError::PasswordHashError)?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| AuthError::InvalidCredentials)
    }

    fn generate_token(&self, user: &User) -> Result<String, AuthError> {
        let now = Utc::now();
        let expiration = now + Duration::hours(self.jwt_expiration_hours);

        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            exp: expiration.timestamp(),
            iat: now.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;

        Ok(token)
    }
}
