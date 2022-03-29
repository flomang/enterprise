use actix_web::{error::BlockingError, post, web, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::Deserialize;

//use crate::email_service::send_invitation;
use crate::utils::errors::ServiceError;
use crate::models::{Invitation, Pool};

#[derive(Deserialize)]
pub struct InvitationData {
    pub email: String,
}

#[post("")]
pub async fn create_invitation(
    invitation_data: web::Json<InvitationData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    // run diesel blocking code
    let res = web::block(move || insert_invitation_and_send(invitation_data.into_inner().email, pool)).await;

    match res {
        Ok(invite) => Ok(HttpResponse::Ok().json(invite)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

fn insert_invitation_and_send(
    eml: String,
    pool: web::Data<Pool>,
) -> Result<Invitation, ServiceError> {
    let invitation = dbg!(query(eml, pool)?);
    
    //send_invitation(&invitation)
    Ok(invitation)
}

/// Diesel query
fn query(eml: String, pool: web::Data<Pool>) -> Result<Invitation, ServiceError> {
    use crate::schema::invitations::dsl::invitations;

    let new_invitation: Invitation = eml.into();
    let conn: &PgConnection = &pool.get().unwrap();

    let inserted_invitation = diesel::insert_into(invitations)
        .values(&new_invitation)
        .get_result(conn)?;

    Ok(inserted_invitation)
}
