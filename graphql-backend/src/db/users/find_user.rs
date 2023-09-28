use super::DbExecutor;
use crate::models::{FindEmail, FindUser, User};
use crate::prelude::*;
use actix::prelude::*;
use diesel::prelude::*;

impl Message for FindUser {
    type Result = Result<User>;
}

impl Handler<FindUser> for DbExecutor {
    type Result = Result<User>;

    fn handle(&mut self, msg: FindUser, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;

        let conn = &mut self.0.get()?;

        let stored_user: User = users.filter(username.eq(msg.username)).first(conn)?;
        Ok(stored_user)
    }
}

impl Message for FindEmail {
    type Result = Result<User>;
}

impl Handler<FindEmail> for DbExecutor {
    type Result = Result<User>;

    fn handle(&mut self, msg: FindEmail, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;

        let conn = &mut self.0.get()?;

        let stored_user: User = users.filter(email.eq(msg.email)).first(conn)?;
        Ok(stored_user)
    }
}
