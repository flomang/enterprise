
use super::DbExecutor;
use actix::prelude::*;
use diesel::prelude::*;
use libreauth::pass::HashBuilder;
use rand::{thread_rng, seq::SliceRandom};
use crate::{prelude::*, graphql::{LoginUser, UserResponse}, models::User};
use crate::utils::{HASHER, PWD_SCHEME_VERSION};

lazy_static! {
    static ref MESSAGES: Vec<&'static str> = vec![
        "Invalid credentials. Please try again.",
        "Login failed. Please check your email and password.",
        "Nope, that's not it. Try again.",
        "Access Denied!",
        "Hmmm...did you mistype something?",
        // Add more messages as needed
    ];
}

fn get_random_message() -> String {
    let mut rng = thread_rng();
    MESSAGES.choose(&mut rng).unwrap().to_string()
}


impl Message for LoginUser {
    type Result = Result<UserResponse>;
}

impl Handler<LoginUser> for DbExecutor {
    type Result = Result<UserResponse>;

    fn handle(&mut self, msg: LoginUser, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;

        let conn = &mut self.0.get()?;

        let stored_user: User = users
            .filter(email.eq(msg.email))
            .first(conn)
            .map_err(|_| Error::Unauthorized(get_random_message()))?;

        let checker = HashBuilder::from_phc(&stored_user.hash)?;
        let provided_password_raw = &msg.password;

        if !checker.is_valid(provided_password_raw) {
            return Err(Error::Unauthorized(get_random_message()));
        }

        if stored_user.email_verified == false {
            return Err(Error::Unauthorized("email not verified".to_string()));
        }

        if checker.needs_update(Some(PWD_SCHEME_VERSION)) {
            let new_password = HASHER.hash(provided_password_raw)?;
            let updated_user = diesel::update(users.find(stored_user.id))
                .set(hash.eq(new_password))
                .get_result::<User>(conn)
                .map_err(|e| Error::from(e))?;

            return Ok(updated_user.into());
        }
        Ok(stored_user.into())
    }
}