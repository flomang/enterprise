use actix_identity::Identity;
use actix_web::{
    dev::Payload, error::BlockingError, get, post, web, Error, FromRequest, HttpRequest, HttpResponse,
};
use diesel::prelude::*;
use diesel::PgConnection;
use futures::future::{err, ok, Ready};
use serde::Deserialize;

use crate::utils::errors::ServiceError;
use crate::models::{Pool, SlimUser, User};
use crate::utils::verify;

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

// we need the same data
// simple aliasing makes the intentions clear and its more readable
pub type LoggedUser = SlimUser;

impl FromRequest for LoggedUser {
    type Config = ();
    type Error = Error;
    type Future = Ready<Result<LoggedUser, Error>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, pl).into_inner() {
            if let Some(user_json) = identity.identity() {
                if let Ok(user) = serde_json::from_str(&user_json) {
                    return ok(user);
                }
            }
        }
        err(ServiceError::Unauthorized.into())
    }
}

#[post("")]
pub async fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Ok().finish()
}

#[post("")]
pub async fn login(
    auth_data: web::Json<AuthData>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || query(auth_data.into_inner(), pool)).await;

    match res {
        Ok(user) => {
            let user_string = serde_json::to_string(&user).unwrap();
            id.remember(user_string);
            Ok(HttpResponse::Ok().json(user))
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

#[get("/")]
pub async fn get_me(id: Identity) -> HttpResponse {
     // access request identity
     if let Some(str) = id.identity() {
        let user: SlimUser = serde_json::from_str(&str).unwrap();
        println!("user: {:?}", user);

        HttpResponse::Ok().json(str)
    } else {
        HttpResponse::Ok().json("Welcome Anonymous!")
    }
}

/// Diesel query
fn query(auth_data: AuthData, pool: web::Data<Pool>) -> Result<SlimUser, ServiceError> {
    use crate::schema::users::dsl::{email, users};
    let conn: &PgConnection = &pool.get().unwrap();
    let mut items = users
        .filter(email.eq(&auth_data.email))
        .load::<User>(conn)?;

    if let Some(user) = items.pop() {
        if let Ok(matching) = verify(&user.hash, &auth_data.password) {
            if matching {
                return Ok(user.into());
            }
        }
    }
    Err(ServiceError::Unauthorized)
}
