use actix_web::{post, web, HttpRequest, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::Deserialize;

//use crate::email_service::send_invitation;
use crate::models::{Invitation, Pool};
use library::errors::ServiceError;
use uuid::Uuid;

pub static KEY: [u8; 16] = *include_bytes!("../secret.key");

#[derive(Deserialize)]
pub struct InvitationData {
    pub email: String,
}


#[post("")]
pub async fn create_invitation(
    request: HttpRequest,
    invitation_data: web::Json<InvitationData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    // must be logged in
    let claims = library::auth::unlock_request(&request, &KEY)?;
    let uid = Uuid::parse_str(&claims.sub).unwrap();

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
