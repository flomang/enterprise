use super::{authenticate_token, AppState};
use crate::models::UserResponse;
use async_graphql::*;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // get the current logged in user by token
    async fn get_current_user<'ctx>(&self, ctx: &Context<'ctx>) -> Result<UserResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let auth = authenticate_token(state, ctx).await?;
        Ok(UserResponse::create_with_auth(auth))
    }

    // get profile for username
    // async fn get_profile<'ctx>(
    //     &self,
    //     ctx: &Context<'ctx>,
    //     username: String,
    // ) -> Result<ProfileResponse> {
    //     let state = ctx.data_unchecked::<AppState>();
    //     let auth = authenticate_token(state, ctx)
    //         .await
    //         .map(|auth| Some(auth))
    //         .unwrap_or(None);

    //     let res = state.db.send(GetProfile { auth, username }).await??;

    //     Ok(res)
    // }
}
