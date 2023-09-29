use crate::{
    models::{UpdateUserOuter, User, UserChange, UserResponse},
    utils::HASHER,
};

use super::DbExecutor;
use crate::prelude::*;
use actix::prelude::*;
use diesel::prelude::*;

impl Message for UpdateUserOuter {
    type Result = Result<UserResponse>;
}

impl Handler<UpdateUserOuter> for DbExecutor {
    type Result = Result<UserResponse>;

    fn handle(&mut self, msg: UpdateUserOuter, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;

        let auth = msg.auth;
        let update_user = msg.update_user;

        let conn = &mut self.0.get()?;

        let updated_hash = match update_user.password {
            Some(updated_password) => Some(HASHER.hash(&updated_password)?),
            None => None,
        };

        let updated_user = UserChange {
            username: update_user.username,
            first_name: update_user.first_name,
            last_name: update_user.last_name,
            email: update_user.email,
            email_verified: update_user.email_verified,
            hash: updated_hash,
        };

        match diesel::update(users.find(auth.user.id))
            .set(&updated_user)
            .get_result::<User>(conn)
        {
            Ok(user) => Ok(user.non_token_response()),
            Err(e) => Err(e.into()),
        }
    }
}
