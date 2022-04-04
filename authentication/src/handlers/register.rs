use actix_web::{post, web, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;

use crate::models::{Invitation, Pool, SlimUser, User};
use kitchen::utils::errors::ServiceError;
use kitchen::utils::hash_password;
// UserData is used to extract data from a post request by the client
#[derive(Debug, Deserialize)]
pub struct UserData {
    pub password: String,
}

#[post("/{invitation_id}")]
pub async fn register_user(
    invitation_id: web::Path<String>,
    user_data: web::Json<UserData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || {
        query(
            invitation_id.into_inner(),
            user_data.into_inner().password,
            pool,
        )
    })
    .await??;
    Ok(HttpResponse::Ok().json(&res))
}

fn query(
    invitation_id: String,
    password: String,
    pool: web::Data<Pool>,
) -> Result<SlimUser, ServiceError> {
    use crate::schema::invitations::dsl::{id, invitations};
    use crate::schema::users::dsl::users;
    let invitation_id = uuid::Uuid::parse_str(&invitation_id)?;

    let conn: &PgConnection = &pool.get().unwrap();
    invitations
        .filter(id.eq(invitation_id))
        .load::<Invitation>(conn)
        .map_err(|_db_error| ServiceError::BadRequest("Invalid Invitation".into()))
        .and_then(|mut result| {
            if let Some(invitation) = result.pop() {
                // if invitation is not expired
                if invitation.expires_at > chrono::Local::now().naive_local() {
                    // try hashing the password, else return the error that will be converted to ServiceError
                    let password: String = hash_password(&password)?;
                    dbg!(&password);
                    let user = User::from_details(invitation.recipient_email, password);
                    let inserted_user: User =
                        diesel::insert_into(users).values(&user).get_result(conn)?;
                    dbg!(&inserted_user);
                    return Ok(inserted_user.into());
                }
            }
            Err(ServiceError::BadRequest("Invalid Invitation".into()))
        })
}