use actix_identity::Identity;
use actix_web::{dev::Payload, post, web, Error, FromRequest, HttpRequest, HttpResponse};
use chrono::prelude::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use std::future::{ready, Ready};
use serde::Deserialize;

use crate::models::{Pool, SlimUser, UpdateUserPassword, User};
use library::utils::errors::ServiceError;
use library::utils::verify;
use library::utils::hash_password;

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
    Ok(HttpResponse::Ok().json(user))
}

/// Diesel query
fn query(auth_data: AuthData, pool: web::Data<Pool>) -> Result<SlimUser, ServiceError> {
    use crate::schema::users::dsl::{email, users};

    let conn: &PgConnection = &pool.get().unwrap();
    let mut people = users
        .filter(email.eq(&auth_data.email))
        .load::<User>(conn)?;

    if let Some(user) = people.pop() {
        // set auth password if not set for master splinter
        if user.email == "master@splinter.com" && user.hash == "" {
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
