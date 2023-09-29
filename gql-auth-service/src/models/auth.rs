use async_graphql::Context;
use async_graphql::Guard;
use async_graphql::async_trait;
use async_graphql::Result;
use crate::graphql::AppState;
use crate::utils::jwt::CanDecodeJwt;

use super::FindUser;
use super::User;

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


use std::str::FromStr;
use validator::ValidationError;
use strum_macros::{Display, EnumString};

#[derive(Eq, PartialEq, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    Master,
    Admin,
    User,
}

impl Role {
    pub fn to_i32(&self) -> i32 {
        match self {
            Role::Master => 1,
            Role::Admin => 2,
            Role::User => 3,
        }
    }

    pub fn from_i32(role_id: i32) -> Role {
        // map these according to the database
        match role_id {
            1 => Role::Master,
            2 => Role::Admin,
            _ => Role::User,  // default to user
        }
    }
}

// validator function that validates a role string
pub fn validate_role(role: &str) -> Result<(), ValidationError> {
    match Role::from_str(&role.to_ascii_uppercase()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("invalid_role")),
    }
}

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