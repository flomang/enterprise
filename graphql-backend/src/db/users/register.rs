use actix::prelude::*;
use diesel::prelude::*;
use crate::prelude::*;

use super::DbExecutor;
use crate::graphql::users::{RegisterUser, UserResponse};
use crate::models::{NewUser, User};
use crate::utils::HASHER;

impl Message for RegisterUser {
    type Result = Result<UserResponse>;
}

impl Handler<RegisterUser> for DbExecutor {
    type Result = Result<UserResponse>;

    fn handle(&mut self, msg: RegisterUser, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;

        let new_user = NewUser {
            username: msg.username.trim().to_string(),
            first_name: msg.first_name.trim().to_string(),
            last_name: msg.last_name.trim().to_string(),
            email: msg.email.clone(),
            hash: HASHER.hash(&msg.password)?,
            role_id: 3,  // todo implement role system
        };

        let conn = &mut self.0.get()?;

        match diesel::insert_into(users)
            .values(new_user)
            .get_result::<User>(conn)
        {
            Ok(user) => Ok(user.into()),
            Err(e) => Err(e.into()),
        }
    }
}
