mod mutation;
mod query;
pub mod server;

use crate::db::DbExecutor;
use crate::models::{Auth, GenerateAuth, Token};
use crate::prelude::*;
use actix::prelude::Addr;
use async_graphql::{EmptySubscription, Schema};

pub type GraphqlSchema = Schema<query::QueryRoot, mutation::MutationRoot, EmptySubscription>;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

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
        Err(_) => Err(Error::Unauthorized(
            "no authorization was provided".to_string(),
        )),
    }
}
