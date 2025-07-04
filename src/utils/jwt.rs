use crate::utils::constants;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
    pub id: i32,
}

pub fn encode_jwt(email: String, id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let expire = Duration::hours(24);

    let claims = Claims {
        exp: (now + expire).timestamp() as usize,
        iat: now.timestamp() as usize,
        email,
        id,
    };

    let secret = constants::SECRET.clone();

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = constants::SECRET.clone();
    let claim_data: Result<TokenData<Claims>, jsonwebtoken::errors::Error> = decode(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    );

    claim_data
}
