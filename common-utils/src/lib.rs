// WARNING: THIS IS ONLY FOR DEMO! PLEASE DO MORE RESEARCH FOR PRODUCTION USE.
use std::str::FromStr;

use actix_web::http::header::ToStrError;
use actix_web::HttpRequest;
use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use strum::ParseError;
use strum_macros::{Display, EnumString};

use std::{env, fmt};

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_connection_pool() -> PgPool {
    let db_url = env::var("DATABASE_URL").expect("Can't get DB URL");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

lazy_static! {
    static ref JWT_SECRET_KEY: String =
        std::env::var("JWT_SECRET_KEY").expect("Can't read JWT_SECRET_KEY");
}

pub const FORBIDDEN_MESSAGE: &str = "Forbidden";

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub role: String,
}

#[derive(Eq, PartialEq, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    Master,
    Admin,
    User,
}

impl Role {
    pub fn to_i32(&self) -> i32 {
        match self {
            Role::Master => 1,
            Role::Admin => 2,
            Role::User => 3,
        }
    }

    pub fn from_i32(role_id: i32) -> Result<Self, CustomError> {
        match role_id {
            1 => Ok(Role::Master),
            2 => Ok(Role::Admin),
            3 => Ok(Role::User),
            _ => Err("Invalid role id".into()),
        }
    }
}

pub fn get_jwt_secret_key() -> String {
    JWT_SECRET_KEY.clone()
}

pub fn create_jwt_token(
    username: String,
    role: Role,
) -> Result<String, jsonwebtoken::errors::Error> {
    let exp_time = Local::now() + Duration::minutes(60);

    let claims = Claims {
        sub: username,
        exp: exp_time.timestamp(),
        role: role.to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET_KEY.as_bytes()),
    )
}

pub fn decode_jwt_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET_KEY.as_bytes()),
        &Validation::default(),
    ) {
        Ok(res) => Ok(res.claims),
        Err(e) => Err(e.into()),
    }
}

pub fn get_role(http_request: HttpRequest) -> Result<Option<Role>, CustomError> {
    let token = http_request.headers().get("Authorization");
    match token {
        Some(t) => {
            let token_str = t.to_str()?;
            let token_str = token_str.replace("Bearer ", "");
            match decode_jwt_token(&token_str) {
                Ok(claims) => Ok(Some(Role::from_str(&claims.role)?)),
                _ => Ok(None),
            }
        }
        _ => Ok(None),
    }
}

pub fn check_user_role_is_allowed(
    getting_role_result: &Result<Option<Role>, CustomError>,
    allowed_role: &Role,
) -> Result<(), CustomError> {
    let maybe_role = match getting_role_result {
        Ok(maybe_role) => maybe_role,
        Err(e) => {
            return Err(format!("Error while getting a user's role: {}", e.message)
                .as_str()
                .into())
        }
    };

    match maybe_role {
        Some(role) => {
            if role == allowed_role {
                Ok(())
            } else {
                Err(FORBIDDEN_MESSAGE.into())
            }
        }
        None => Err(FORBIDDEN_MESSAGE.into()),
    }
}

#[derive(Debug)]
pub struct CustomError {
    pub message: String,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom Error: {}", self.message)
    }
}

impl From<ToStrError> for CustomError {
    fn from(source: ToStrError) -> Self {
        Self {
            message: source.to_string(),
        }
    }
}

impl From<ParseError> for CustomError {
    fn from(source: ParseError) -> Self {
        Self {
            message: source.to_string(),
        }
    }
}

impl From<&str> for CustomError {
    fn from(source: &str) -> Self {
        Self {
            message: String::from(source),
        }
    }
}

use chrono::NaiveDateTime;
use serde::Serializer;

// The Serialize trait is not impl'd for NaiveDateTime
// This is a custom wrapper type to get around that
#[derive(Debug, PartialEq)]
pub struct CustomDateTime(pub NaiveDateTime);

impl Serialize for CustomDateTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let s = self.0.format("%Y-%m-%dT%H:%M:%S.%3fZ");
        serializer.serialize_str(&s.to_string())
    }
}
