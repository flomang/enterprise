use actix::prelude::*;
use diesel::prelude::*;
use crate::db::DbExecutor;
use crate::models::{GenerateAuth, Auth};
use crate::prelude::*;
use crate::utils::jwt::CanDecodeJwt;

impl Message for GenerateAuth {
    type Result = Result<Auth>;
}

impl Handler<GenerateAuth> for DbExecutor {
    type Result = Result<Auth>;

    fn handle(&mut self, msg: GenerateAuth, _: &mut Self::Context) -> Self::Result {
        use super::schema::users::dsl::*;

        let claims = msg.token.decode_jwt()?.claims;

        let conn = &mut self.0.get()?;

        match users.find(claims.id).first(conn) {
            Ok(user) => Ok(Auth {
                user,
                token: msg.token,
            }),
            Err(e) => Err(e.into()),
        }
    }
}
