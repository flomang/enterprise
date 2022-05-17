use actix_identity::Identity;
use actix_web::{post, web, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::Deserialize;

//use crate::email_service::send_invitation;
use crate::models::{Invitation, Pool, SlimUser};
use library::errors::ServiceError;

#[derive(Deserialize)]
pub struct InvitationData {
    pub email: String,
}

#[post("")]
pub async fn create_invitation(
    invitation_data: web::Json<InvitationData>,
    identity: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    // must be logged in
    match identity.identity() {
        Some(str) => {
            let user: SlimUser = serde_json::from_str(&str).unwrap();
            let result = web::block(move || {
                insert_invitation_and_send(user.id, invitation_data.into_inner().email, pool)
            })
            .await??;
            Ok(HttpResponse::Ok().json(result))
        }
        None => Err(ServiceError::Unauthorized),
    }
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
