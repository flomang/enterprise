use super::DbExecutor;
use crate::models::{FindEmail, FindUser, User, UserResponse};
use crate::prelude::*;
use actix::prelude::*;
use diesel::prelude::*;

impl Message for FindUser {
    type Result = Result<UserResponse>;
}

impl Handler<FindUser> for DbExecutor {
    type Result = Result<UserResponse>;

    fn handle(&mut self, msg: FindUser, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;

        let conn = &mut self.0.get()?;

        let stored_user: User = users.filter(username.eq(msg.username)).first(conn)?;
        Ok(stored_user.non_token_response())
    }
}

impl Message for FindEmail {
    type Result = Result<UserResponse>;
}

impl Handler<FindEmail> for DbExecutor {
    type Result = Result<UserResponse>;

    fn handle(&mut self, msg: FindEmail, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;

        let conn = &mut self.0.get()?;

        let stored_user: User = users.filter(email.eq(msg.email)).first(conn)?;
        Ok(stored_user.non_token_response())
    }
}
