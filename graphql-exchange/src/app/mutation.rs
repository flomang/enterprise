use async_graphql::*;
use validator::Validate;

use crate::{
    app::{
        users::{LoginUser, RegisterUser, UpdateUser, UpdateUserOuter, UserResponse},
        AppState,
    },
    utils::auth::authenticate_token,
};

use super::{
    profiles::{FollowProfile, ProfileResponse, UnfollowProfile},
    Token,
};
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // register a new user
    async fn signup<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        params: RegisterUser,
    ) -> Result<UserResponse> {
        params.validate()?;

        let state = ctx.data_unchecked::<AppState>();
        let res = state.db.send(params).await??;
        Ok(res)
    }

    // login a user
    async fn signin<'ctx>(&self, ctx: &Context<'ctx>, params: LoginUser) -> Result<UserResponse> {
        params.validate()?;

        let state = ctx.data_unchecked::<AppState>();
        let res = state.db.send(params).await??;
        Ok(res)
    }

    // update a user
    async fn update_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        params: UpdateUser,
    ) -> Result<UserResponse> {
        params.validate()?;

        let state = ctx.data_unchecked::<AppState>();
        let token = ctx.data::<Token>()?.0.clone();
        let auth = authenticate_token(state, token).await?;
        let res = state
            .db
            .send(UpdateUserOuter {
                auth,
                update_user: params,
            })
            .await??;
        Ok(res)
    }

    // follow a user
    async fn follow_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        username: String,
    ) -> Result<ProfileResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let token = ctx.data::<Token>()?.0.clone();
        let auth = authenticate_token(state, token).await?;
        let res = state.db.send(FollowProfile { auth, username }).await??;
        Ok(res)
    }

    // unfollow a user
    async fn unfollow_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        username: String,
    ) -> Result<ProfileResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let token = ctx.data::<Token>()?.0.clone();
        let auth = authenticate_token(state, token).await?;
        let res = state.db.send(UnfollowProfile { auth, username }).await??;
        Ok(res)
    }
}
