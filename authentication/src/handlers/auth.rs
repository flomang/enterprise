use actix_identity::Identity;
use actix_web::{
    dev::Payload, dev::ServiceRequest, post, web, Error, FromRequest, HttpRequest, HttpResponse,
};
use diesel::prelude::*;
use diesel::PgConnection;
use std::future::{ready, Ready};

use crate::models::{Pool, SlimUser, UpdateUserPassword, User};
use kitchen::utils::errors::ServiceError;
use kitchen::utils::hash_password;
use kitchen::utils::verify;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;

#[derive(Deserialize, Serialize, Debug)]
struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
    username: String,
}

pub async fn bearer_auth_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);
    if validate_token(credentials.token()) {
        Ok(req)
    } else {
        Err(AuthenticationError::from(config).into())
    }
}

fn validate_token(token: &str) -> bool {
    let key = std::env::var("JWT_KEY").unwrap_or_else(|_| "0123".repeat(8));
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(key.as_bytes()),
        &Validation::default(),
    ) {
        Ok(c) => true,
        Err(err) => {
            log::info!("err: {:?}", err.kind());
            false
        }
    }
}

fn create_jwt(username: String) -> Result<String, ServiceError> {
    let key = std::env::var("JWT_KEY").unwrap_or_else(|_| "0123".repeat(8));
    let sub = std::env::var("SUBDOMAIN").unwrap_or_else(|_| "h@d.com".to_string());
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
        sub,
        iat: my_iat as usize,
        exp: my_exp as usize,
        username,
    };

    match encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(key.as_bytes()),
    ) {
        Ok(t) => Ok(t),
        Err(err) => {
            log::error!("create_jwt: {}", err);
            Err(ServiceError::InternalServerError)
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

// we need the same data
// simple aliasing makes the intentions clear and its more readable
pub type LoggedUser = SlimUser;

impl FromRequest for LoggedUser {
    type Error = Error;
    type Future = Ready<Result<LoggedUser, Error>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, pl).into_inner() {
            if let Some(user_json) = identity.identity() {
                if let Ok(user) = serde_json::from_str(&user_json) {
                    return ready(Ok(user));
                }
            }
        }

        ready(Err(ServiceError::Unauthorized.into()))
    }
}

#[post("")]
pub async fn logout(identity: Identity) -> HttpResponse {
    identity.forget();
    HttpResponse::Ok().finish()
}

#[post("")]
pub async fn login(
    auth_data: web::Json<AuthData>,
    identity: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let user = web::block(move || query(auth_data.into_inner(), pool)).await??;
    let user_string = serde_json::to_string(&user).unwrap();
    identity.remember(user_string);

    let token = create_jwt(user.username.clone())?;

    let session = Session {
        user_id: user.id,
        email: user.email,
        username: user.username,
        avatar_url: user.avatar_url,
        token,
    };

    Ok(HttpResponse::Ok().json(session))
}

/// Diesel query
fn query(auth_data: AuthData, pool: web::Data<Pool>) -> Result<SlimUser, ServiceError> {
    use crate::schema::users::dsl::{email, users};

    let conn: &PgConnection = &pool.get().unwrap();
    let mut people = users
        .filter(email.eq(&auth_data.email))
        .load::<User>(conn)?;

    if let Some(user) = people.pop() {
        let master = std::env::var("MASTER_EMAIL").expect("MASTER_EMAIL must be set");
        // set auth password if not set for master
        if user.email == master && user.hash == "" {
            let now = Utc::now().naive_utc();
            let password = hash_password(&auth_data.password)?;
            let set_pwd = UpdateUserPassword {
                id: user.id,
                hash: password,
                updated_at: now,
            };

            let result = diesel::update(users).set(&set_pwd).get_result::<User>(conn);

            match result {
                Ok(u) => return Ok(u.into()),
                Err(e) => return Err(ServiceError::BadRequest(e.to_string())),
            }
        } else if let Ok(matching) = verify(&user.hash, &auth_data.password) {
            if matching {
                return Ok(user.into());
            }
        }
    }
    Err(ServiceError::Unauthorized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let token = create_jwt("master splinter".to_string());

        println!("{:?}", token);
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
