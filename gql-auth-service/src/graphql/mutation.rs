use super::{authenticate_token, AppState};
use crate::error::validation_errors_to_error;
use crate::models::{
    ForgotPassword, LoginUser, RegisterUser, UpdateUser, UpdateUserOuter, UpdateUserRole,
    UserResponse,
};
use crate::models::{Role, RoleGuard};
use async_graphql::*;
use validator::{Validate, ValidateArgs};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // register a new user
    async fn signup<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        params: RegisterUser,
    ) -> Result<UserResponse> {
        let state = ctx.data_unchecked::<AppState>();

        params
            .validate_args((state, state))
            .map_err(|e| validation_errors_to_error(e).extend())?;

        let res = state.db.send(params).await??;
        Ok(res)
    }

    // forgot password
    async fn forgot_password<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        params: ForgotPassword,
    ) -> Result<bool> {
        let state = ctx.data_unchecked::<AppState>();

        params
            .validate_args(state)
            .map_err(|e| validation_errors_to_error(e).extend())?;

        // TODO send email with reset link
        Ok(true)
    }

    // login a user
    async fn signin<'ctx>(&self, ctx: &Context<'ctx>, params: LoginUser) -> Result<UserResponse> {
        params
            .validate()
            .map_err(|e| validation_errors_to_error(e).extend())?;

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
        let state = ctx.data_unchecked::<AppState>();
        let auth = authenticate_token(state, ctx).await?;

        params
            .validate_args(state)
            .map_err(|e| validation_errors_to_error(e).extend())?;

        let res = state
            .db
            .send(UpdateUserOuter {
                auth,
                update_user: params,
            })
            .await??;
        Ok(res)
    }

    // update role
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn update_role<'ctx>(&self, ctx: &Context<'ctx>, params: UpdateUserRole) -> Result<UserResponse> {
        let state = ctx.data_unchecked::<AppState>();
        let _auth = authenticate_token(state, ctx).await?;

        params
            .validate_args(state)
            .map_err(|e| validation_errors_to_error(e).extend())?;

        let res = state.db.send(params).await??;
        Ok(res)
    }
}
