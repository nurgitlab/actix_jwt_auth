use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode, encode,
};
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    errors::{auth_errors::AuthError, users_errors::UserError},
    models::auth_models::{
        Claims, LoginRequest, RefreshRequest, RefreshToken, TokenPair,
    },
    repositories::{
        auth_repisitory::AuthRepository, users_repository::UserRepository,
    },
};

const JWT_SECRET: &[u8] = b"your_secret_key";

pub struct AuthService;

impl AuthService {
    pub async fn login(
        pool: &PgPool,
        credentials: LoginRequest,
    ) -> Result<TokenPair, AuthError> {
        let user_id = Self::authenticate_user(
            pool,
            &credentials.username,
            &credentials.password,
        )
        .await?;

        // Генерация токенов
        let token_pair = Self::generate_token_pair(user_id)?;
        let refresh_token = RefreshToken::new(user_id);

        // Сохранение refresh token в БД
        AuthRepository::save_refresh_token(pool, &refresh_token).await?;

        Ok(TokenPair {
            access_token: token_pair.access_token,
            refresh_token: refresh_token.token,
        })
    }

    pub async fn refresh(
        pool: &PgPool,
        token_data: RefreshRequest,
    ) -> Result<TokenPair, AuthError> {
        // Проверяем валидность refresh token
        let user_id = AuthRepository::validate_refresh_token(
            pool,
            &token_data.refresh_token,
        )
        .await?;

        // Удаляем использованный refresh token
        AuthRepository::delete_refresh_token(pool, &token_data.refresh_token)
            .await?;

        // Генерируем новую пару токенов
        let token_pair = Self::generate_token_pair(user_id)?;
        let new_refresh_token = RefreshToken::new(user_id);

        // Сохраняем новый refresh token
        AuthRepository::save_refresh_token(pool, &new_refresh_token).await?;

        Ok(TokenPair {
            access_token: token_pair.access_token,
            refresh_token: new_refresh_token.token,
        })
    }

    pub async fn logout(
        pool: &PgPool,
        token_data: RefreshRequest,
    ) -> Result<(), AuthError> {
        // Удаляем refresh token из БД
        AuthRepository::delete_refresh_token(pool, &token_data.refresh_token)
            .await
    }

    // Вспомогательная функция для аутентификации пользователя
    pub async fn authenticate_user(
        pool: &PgPool,
        username: &str,
        password: &str,
    ) -> Result<i32, AuthError> {
        if username.is_empty() || password.is_empty() {
            return Err(AuthError::Authentication(
                "Username and password are required".to_string(),
            ));
        }

        let user = UserRepository::find_by_username(pool, username)
            .await
            .map_err(|e| {
                AuthError::Authentication(format!(
                    "Authentication failed: {}",
                    e
                ))
            })?;

        if username == user.username && password == user.password {
            Ok(user.id)
        } else {
            Err(AuthError::Authentication("Invalid credentials".to_string()))
        }
    }

    pub fn generate_token_pair(user_id: i32) -> Result<TokenPair, AuthError> {
        let claims = Claims::new(user_id);
        let refresh_token = RefreshToken::new(user_id);

        let access_token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(JWT_SECRET),
        )
        .map_err(AuthError::InvalidToken)?;

        Ok(TokenPair { access_token, refresh_token: refresh_token.token })
    }

    pub fn validate_access_token(token: &str) -> Result<Claims, AuthError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(JWT_SECRET),
            &Validation::default(),
        )
        .map(|token_data| {
            log::debug!(
                "Access token validated for user {}",
                token_data.claims.sub
            );
            token_data.claims
        })
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                log::warn!("Access token expired");
                AuthError::TokenExpired
            }
            _ => {
                log::warn!("Invalid access token: {}", e);
                AuthError::InvalidToken(e)
            }
        })
    }
}
