use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DecodedClaims {
    pub username: String,
}

pub fn decode_jwt(token: &str) -> Result<DecodedClaims, String> {
    let secret_key = std::env::var("JWT_SECRET")
        .map_err(|_| "JWT_SECRET environment variable not set")?;

    let token_data = decode::<DecodedClaims>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::default(),
    );

    match token_data {
        Ok(token_data) => Ok(token_data.claims),

        Err(e) => Err(e.to_string()),
    }
}
