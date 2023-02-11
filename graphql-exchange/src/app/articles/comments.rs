use actix_web::{web::Data, web::Json, web::Path, HttpRequest, HttpResponse};
use validator::Validate;

use super::super::AppState;
use crate::app::profiles::ProfileResponseInner;
use crate::prelude::*;
use crate::utils::{
    auth::{authenticate, Auth},
    CustomDateTime,
};

#[derive(Debug, Deserialize)]
pub struct In<T> {
    comment: T,
}

// Extractors ↓

use super::ArticlePath;

#[derive(Debug, Deserialize)]
pub struct ArticleCommentPath {
    slug: String,
    comment_id: i32,
}

// Client Messages ↓

#[derive(async_graphql::InputObject)]
#[derive(Debug, Validate, Deserialize)]
pub struct AddComment {
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub body: String,
}

#[derive(Debug)]
pub struct AddCommentOuter {
    pub auth: Auth,
    pub slug: String,
    pub comment: AddComment,
}

#[derive(Debug)]
pub struct GetComments {
    pub auth: Option<Auth>,
    pub slug: String,
}

#[derive(Debug)]
pub struct DeleteComment {
    pub auth: Auth,
    pub slug: String,
    pub comment_id: i32,
}

// JSON response objects ↓

#[derive(async_graphql::SimpleObject)]
#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub comment: CommentResponseInner,
}

#[derive(async_graphql::SimpleObject)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentResponseInner {
    pub id: i32,
    pub created_at: CustomDateTime,
    pub updated_at: CustomDateTime,
    pub body: String,
    pub author: ProfileResponseInner,
}

#[derive(async_graphql::SimpleObject)]
#[derive(Debug, Serialize)]
pub struct CommentListResponse {
    pub comments: Vec<CommentResponseInner>,
}
