use actix_web::{
    dev::ServiceRequest, Error,
};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use argonautica::{Hasher, Verifier};
use chrono::Utc;
use crate::errors::ServiceError;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    // the subject will be the user-id
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
    pub username: String,
}

// Note: bearer_auth_validator returns Error instead of ServiceError
// this is intentional to conform to HttpAuthentication::bearer sig.
pub async fn bearer_auth_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    let key = std::env::var("JWT_KEY").unwrap_or_else(|_| "0123".repeat(8));
    if let Ok(_) = validate_token(credentials.token(), &key.as_bytes()) {
       Ok(req)
    } else {
        Err(AuthenticationError::from(config).into())
    }
}

pub fn validate_token(token: &str, secret: &[u8]) -> Result<Claims, ServiceError> {
    let validation = Validation::new(Algorithm::HS256);

    let data = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &validation,
    ).map_err(|err| {
        dbg!(err);
        ServiceError::Unauthorized
    })?;

    Ok(data.claims)
}

pub fn create_jwt(user_id: Uuid, username: String, secret: &[u8]) -> Result<String, ServiceError> {
    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
    let payload = Claims {
            sub: user_id.to_string(),
            iat: now,
            exp: now + ONE_WEEK,
            username,
        };

    jsonwebtoken::encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(&secret),
    )
    .map_err(|err| {
        dbg!(err);
        ServiceError::InternalServerError
    })
}

lazy_static::lazy_static! {
pub  static ref SECRET_KEY: String = std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
}

// WARNING THIS IS ONLY FOR DEMO PLEASE DO MORE RESEARCH FOR PRODUCTION USE
pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    Hasher::default()
        .with_password(password)
        .with_secret_key(SECRET_KEY.as_str())
        .hash()
        .map_err(|err| {
            dbg!(err);
            ServiceError::InternalServerError
        })
}

pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    Verifier::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(SECRET_KEY.as_str())
        .verify()
        .map_err(|err| {
            dbg!(err);
            ServiceError::Unauthorized
        })
}