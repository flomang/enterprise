use async_graphql::Context;
use async_graphql::Guard;
use async_graphql::async_trait;
use async_graphql::Result;
use crate::graphql::AppState;
use crate::utils::jwt::CanDecodeJwt;

use super::FindUser;
use super::{User, Role};

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


pub struct RoleGuard {
    role: Role,
}

impl RoleGuard {
    pub fn new(role: Role) -> Self {
        Self { role }
    }
}

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let token = ctx.data::<Token>()?;
        let data = token.0.decode_jwt()?;
        let state = ctx.data_unchecked::<AppState>();
        let user = state.db.send(FindUser{username: data.claims.sub}).await??;

        if Role::from_i32(user.role_id) == self.role {
            Ok(())
        } else {
            Err(format!("user role is not: {}", self.role).into())
        }
    }
}