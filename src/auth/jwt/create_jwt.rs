use jsonwebtoken::{EncodingKey, Header, encode, errors::Error};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(Deserialize, Serialize)]
struct Claims {
    username: String,
    exp: i64,
}

pub fn create_jwt(username: String) -> Result<String, String> {
    let exp_time = OffsetDateTime::now_utc() + Duration::minutes(1);
    let exp_timestamp = exp_time.unix_timestamp(); //to i64

    let secret_key = std::env::var("JWT_SECRET")
        .map_err(|_| "JWT_SECRET environment variable not set")?;

    let token = encode(
        &Header::default(),
        &Claims { username: username, exp: exp_timestamp },
        &EncodingKey::from_secret(secret_key.as_bytes()),
    )
    .map_err(|e: Error| e.to_string())?;

    Ok(token)
}
