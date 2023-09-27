use crate::graphql::AppState;
use crate::models::User;
use crate::prelude::*;

// expand this as needed
#[derive(Debug)]
pub struct Auth {
    pub user: User,
    pub token: String,
}

// create auth message
#[derive(Debug)]
pub struct GenerateAuth {
    pub token: String,
}

pub struct Token(pub String);

pub async fn authenticate_token<'ctx>(
    state: &AppState,
    ctx: &async_graphql::Context<'ctx>,
) -> Result<Auth, Error> {
    let token = ctx.data::<Token>();
    match token {
        Ok(token) => {
            let token = token.0.clone();
            let auth = state.db.send(GenerateAuth { token }).await??;
            Ok(auth)
        }
        Err(_) => Err(Error::Unauthorized("no authorization was provided".to_string())),
    }
}
