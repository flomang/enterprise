use super::DbExecutor;
use crate::models::{Role, UpdateUserRole, User, UserResponse};
use crate::prelude::*;
use actix::prelude::*;
use diesel::prelude::*;
use std::str::FromStr;

impl Message for UpdateUserRole {
    type Result = Result<UserResponse>;
}

impl Handler<UpdateUserRole> for DbExecutor {
    type Result = Result<UserResponse>;

    fn handle(&mut self, msg: UpdateUserRole, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;

        let conn = &mut self.0.get()?;

        // TODO handle unwrap more gracefully here 
        let role = Role::from_str(&msg.role.to_ascii_uppercase()).unwrap().to_i32();
        let stored_user: User = users.filter(username.eq(msg.username)).first(conn)?;

        match diesel::update(&stored_user)
            .set(role_id.eq(role))
            .get_result::<User>(conn)
        {
            Ok(user) => Ok(user.non_token_response()),
            Err(e) => Err(e.into()),
        }
    }
}
