use argonautica::{Hasher, Verifier};
use bigdecimal::{BigDecimal, ToPrimitive};
use serde::ser::Serializer;

pub mod errors;
pub mod pagination;
pub mod auth;

use errors::ServiceError;

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

pub fn serialize_bigdecimal_opt<S>(bg: &Option<BigDecimal>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match bg {
     Some(b) => serializer.serialize_f64(b.to_f64().unwrap()),
     None => serializer.serialize_none(),
    }
}

pub fn serialize_bigdecimal<S>(bg: &BigDecimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_f64(bg.to_f64().unwrap())
}