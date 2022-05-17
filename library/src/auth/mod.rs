use actix_web::{
    dev::ServiceRequest, Error,
};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use argonautica::{Hasher, Verifier};
use chrono::{Duration, Utc};
use crate::errors::ServiceError;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    // the subject will be the user-id
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub username: String,
}

pub async fn bearer_auth_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    if let Ok(_) = validate_token(credentials.token()) {
       Ok(req)
    } else {
        Err(AuthenticationError::from(config).into())
    }
}

pub fn validate_token(token: &str) -> Result<Claims, ServiceError> {
    let key = std::env::var("JWT_KEY").unwrap_or_else(|_| "0123".repeat(8));
    let validation = Validation::new(Algorithm::HS256);

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(key.as_bytes()),
        &validation,
    ).map_err(|err| {
        dbg!(err);
        ServiceError::Unauthorized
    })?;

    Ok(data.claims)
}

pub fn create_jwt(user_id: Uuid, username: String) -> Result<String, ServiceError> {
    let key = std::env::var("JWT_KEY").unwrap_or_else(|_| "0123".repeat(8));
    let hours: i64 = std::env::var("JWT_HOURS")
        .unwrap_or_else(|_| "24".to_string())
        .parse()
        .unwrap();

    let my_iat = Utc::now().timestamp();
    let my_exp = Utc::now()
        .checked_add_signed(Duration::hours(hours))
        .expect("invalid timestamp")
        .timestamp();

    let my_claims = Claims {
        sub: user_id.to_string(),
        iat: my_iat as usize,
        exp: my_exp as usize,
        username: username,
    };

    encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(key.as_bytes()),
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