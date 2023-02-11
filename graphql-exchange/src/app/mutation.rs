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
    articles::{
        ArticleResponse, CreateArticle, CreateArticleOuter, DeleteArticle, FavoriteArticle,
        UpdateArticle, UpdateArticleOuter, UnfavoriteArticle, comments::{AddCommentOuter, AddComment, CommentResponse, DeleteComment},
    },
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

    // // follow a user
    // async fn follow_user<'ctx>(
    //     &self,
    //     ctx: &Context<'ctx>,
    //     username: String,
    // ) -> Result<ProfileResponse> {
    //     let state = ctx.data_unchecked::<AppState>();
    //     let token = ctx.data::<Token>()?.0.clone();
    //     let auth = authenticate_token(state, token).await?;
    //     let res = state.db.send(FollowProfile { auth, username }).await??;
    //     Ok(res)
    // }

    // // unfollow a user
    // async fn unfollow_user<'ctx>(
    //     &self,
    //     ctx: &Context<'ctx>,
    //     username: String,
    // ) -> Result<ProfileResponse> {
    //     let state = ctx.data_unchecked::<AppState>();
    //     let token = ctx.data::<Token>()?.0.clone();
    //     let auth = authenticate_token(state, token).await?;
    //     let res = state.db.send(UnfollowProfile { auth, username }).await??;
    //     Ok(res)
    // }

    // // create article
    // async fn create_acticle<'ctx>(
    //     &self,
    //     ctx: &Context<'ctx>,
    //     params: CreateArticle,
    // ) -> Result<ArticleResponse> {
    //     params.validate()?;

    //     let state = ctx.data_unchecked::<AppState>();
    //     let token = ctx.data::<Token>()?.0.clone();
    //     let auth = authenticate_token(state, token).await?;
    //     let res = state
    //         .db
    //         .send(CreateArticleOuter {
    //             auth,
    //             article: params,
    //         })
    //         .await??;

    //     Ok(res)
    // }

    // // update article
    // async fn update_acticle<'ctx>(
    //     &self,
    //     ctx: &Context<'ctx>,
    //     slug: String,
    //     params: UpdateArticle,
    // ) -> Result<ArticleResponse> {
    //     params.validate()?;

    //     let state = ctx.data_unchecked::<AppState>();
    //     let token = ctx.data::<Token>()?.0.clone();
    //     let auth = authenticate_token(state, token).await?;
    //     let res = state
    //         .db
    //         .send(UpdateArticleOuter {
    //             auth,
    //             slug,
    //             article: params,
    //         })
    //         .await??;

    //     Ok(res)
    // }

    // // update article
    // async fn delete_acticle<'ctx>(&self, ctx: &Context<'ctx>, slug: String) -> Result<bool> {
    //     let state = ctx.data_unchecked::<AppState>();
    //     let token = ctx.data::<Token>()?.0.clone();
    //     let auth = authenticate_token(state, token).await?;
    //     state.db.send(DeleteArticle { auth, slug }).await??;
    //     Ok(true)
    // }

    // // favorite article
    // async fn favorite_acticle<'ctx>(
    //     &self,
    //     ctx: &Context<'ctx>,
    //     slug: String,
    // ) -> Result<ArticleResponse> {
    //     let state = ctx.data_unchecked::<AppState>();
    //     let token = ctx.data::<Token>()?.0.clone();
    //     let auth = authenticate_token(state, token).await?;
    //     let res = state.db.send(FavoriteArticle { auth, slug }).await??;
    //     Ok(res)
    // }

    // // unfavorite article
    // async fn unfavorite_acticle<'ctx>(
    //     &self,
    //     ctx: &Context<'ctx>,
    //     slug: String,
    // ) -> Result<ArticleResponse> {
    //     let state = ctx.data_unchecked::<AppState>();
    //     let token = ctx.data::<Token>()?.0.clone();
    //     let auth = authenticate_token(state, token).await?;
    //     let res = state.db.send(UnfavoriteArticle { auth, slug }).await??;
    //     Ok(res)
    // }

    // // add comment to article
    // async fn add_comment<'ctx>(
    //     &self,
    //     ctx: &Context<'ctx>,
    //     slug: String,
    //     comment: AddComment,
    // ) -> Result<CommentResponse> {
    //     comment.validate()?;

    //     let state = ctx.data_unchecked::<AppState>();
    //     let token = ctx.data::<Token>()?.0.clone();
    //     let auth = authenticate_token(state, token).await?;
    //     let res = state
    //         .db
    //         .send(AddCommentOuter {
    //             auth,
    //             slug,
    //             comment
    //         })
    //         .await??;
    //     Ok(res)
    // }

    // // delete comment from article
    // async fn delete_comment<'ctx>(
    //     &self,
    //     ctx: &Context<'ctx>,
    //     slug: String,
    //     comment_id: i32,
    // ) -> Result<bool> {
    //     let state = ctx.data_unchecked::<AppState>();
    //     let token = ctx.data::<Token>()?.0.clone();
    //     let auth = authenticate_token(state, token).await?;
    //     state.db.send(DeleteComment { auth, slug, comment_id }).await??;
    //     Ok(true)
    // }
}
