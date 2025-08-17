use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{errors::auth_errors::AuthError, models::auth_models::{Claims, LoginRequest, RefreshRequest, RefreshToken, TokenPair}, repositories::auth_repisitory::AuthRepository};

const JWT_SECRET: &[u8] = b"your_secret_key";

pub struct AuthService;

impl AuthService {
    pub async fn login(
        pool: &PgPool,
        credentials: LoginRequest,
    ) -> Result<TokenPair, AuthError> {
        // В реальном приложении здесь должна быть проверка учетных данных
        // Для примера просто генерируем токены для любого валидного запроса
        
        // Аутентификация пользователя (заглушка)
        let user_id = Self::authenticate_user(&credentials.username, &credentials.password)?;
        
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
        let user_id = AuthRepository::validate_refresh_token(pool, &token_data.refresh_token).await?;
        
        // Удаляем использованный refresh token
       AuthRepository::delete_refresh_token(pool, &token_data.refresh_token).await?;
        
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
       AuthRepository::delete_refresh_token(pool, &token_data.refresh_token).await
    }
    
    // Вспомогательная функция для аутентификации пользователя
    fn authenticate_user(username: &str, password: &str) -> Result<i32, AuthError> {
        // Заглушка - в реальном приложении здесь должна быть проверка в БД
        if username.is_empty() || password.is_empty() {
            return Err(AuthError::Authentication("Username and password are required".to_string()));
        }
        
        // Эмуляция проверки пользователя
        if username == "admin@example.com" && password == "password" {
            Ok(1) // Возвращаем ID пользователя
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
        ).map_err(AuthError::InvalidToken)?;
        
        Ok(TokenPair {
            access_token,
            refresh_token: refresh_token.token,
        })
    }

    pub fn validate_access_token(token: &str) -> Result<Claims, AuthError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(JWT_SECRET),
            &Validation::default(),
        )
        .map(|token_data| {
            log::debug!("Access token validated for user {}", token_data.claims.sub);
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