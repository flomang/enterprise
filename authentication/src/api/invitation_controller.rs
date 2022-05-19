use actix_web::{post, web, HttpRequest, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::Deserialize;

//use crate::email_service::send_invitation;
use crate::models::{Invitation, Pool};
use library::auth::validate_token;
use library::errors::ServiceError;
use uuid::Uuid;

pub static KEY: [u8; 16] = *include_bytes!("../secret.key");

#[derive(Deserialize)]
pub struct InvitationData {
    pub email: String,
}

pub fn get_uid_from_request(request: &HttpRequest) -> Result<Uuid, ServiceError> {
    let authen_header = match request.headers().get("Authorization") {
        Some(authen_header) => authen_header,
        None => {
            return Err(ServiceError::BadRequest(
                "no Authorization header".to_string(),
            ));
        }
    };

    match authen_header.to_str() {
        Ok(authen_str) => {
            if !authen_str.starts_with("bearer") && !authen_str.starts_with("Bearer") {
                return Err(ServiceError::Unauthorized);
            }

            let raw_token = authen_str[6..authen_str.len()].trim();
            let token = validate_token(&raw_token.to_string(), &KEY)?;
            let uid = Uuid::parse_str(&token.sub).unwrap();
            Ok(uid)
        }
        Err(err) => {
            log::error!("{}", err);
            return Err(ServiceError::InternalServerError);
        }
    }
}

#[post("")]
pub async fn create_invitation(
    request: HttpRequest,
    invitation_data: web::Json<InvitationData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    // must be logged in
    let uid = get_uid_from_request(&request)?;

    //let user: SlimUser = serde_json::from_str(&str).unwrap();
    let result = web::block(move || {
        insert_invitation_and_send(uid, invitation_data.into_inner().email, pool)
    })
    .await??;

    Ok(HttpResponse::Ok().json(result))
}

fn insert_invitation_and_send(
    sender_id: uuid::Uuid,
    eml: String,
    pool: web::Data<Pool>,
) -> Result<Invitation, ServiceError> {
    let invitation = dbg!(query(sender_id, eml, pool)?);

    //send_invitation(&invitation)
    Ok(invitation)
}

/// Diesel query
fn query(
    sender_id: uuid::Uuid,
    eml: String,
    pool: web::Data<Pool>,
) -> Result<Invitation, ServiceError> {
    use crate::schema::invitations::dsl::invitations;

    let new_invitation: Invitation = Invitation::new(sender_id, eml);
    let conn: &PgConnection = &pool.get().unwrap();

    let inserted_invitation = diesel::insert_into(invitations)
        .values(&new_invitation)
        .get_result(conn)?;

    Ok(inserted_invitation)
}
