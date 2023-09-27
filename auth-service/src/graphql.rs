use std::borrow::Cow;
use std::collections::HashMap;

use async_graphql::*;
use failure::Fail;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use common_utils::{CustomError, FORBIDDEN_MESSAGE};
use validator::{Validate, ValidateArgs, ValidationErrors};
use serde_json::Value as JsonValue;

use crate::db::model::{NewUserEntity, UserEntity};
// use crate::db::repository;
//use crate::db::repository as db;
use crate::{get_conn_from_ctx, AuthRole, AppState};
use crate::{hash_password, verify_password};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
    async fn get_users(&self, ctx: &Context<'_>) -> Vec<User> {
        // db::get_all(&mut get_conn_from_ctx(ctx))
        //     .expect("Can't get planets")
        //     .iter()
        //     .map(User::from)
        //     .collect()
        vec![]
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    // #[graphql(guard = "RoleGuard::new(AuthRole::Admin)")]
    // async fn create_user(&self, ctx: &Context<'_>, user: UserInput) -> Result<User> {
    //     let new_user = NewUserEntity {
    //         email: user.email,
    //         username: user.username,
    //         hash: hash_password(user.password.as_str())?,
    //         first_name: user.first_name,
    //         last_name: user.last_name,
    //         role_id: AuthRole::User.to_i32(),
    //     };

    //     let created_user_entity = db::create(new_user, &mut get_conn_from_ctx(ctx))?;

    //     Ok(User::from(&created_user_entity))
    // }

    // async fn sign_in(&self, ctx: &Context<'_>, input: SignInInput) -> Result<String> {
    //     let user = db::get_user(&input.username, &mut get_conn_from_ctx(ctx))?;
    //     verify_password(&user.hash, &input.password)?;
    //     let role = AuthRole::from_i32(user.role_id)?;
    //     let new_token = common_utils::create_jwt_token(user.username, role)?;
    //     Ok(new_token)
    // }

    // register a new user
     async fn signup<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        params: RegisterUser,
    ) -> Result<String> {
        // let state = ctx.data_unchecked::<AppState>();

        // params
        //     .validate_args((state, state))
        //     .map_err(|e| validation_errors_to_error(e).extend())?;

        // let res = state.db.send(params).await??;
        Ok("ok".to_string())
    }
}


#[derive(async_graphql::InputObject, Debug, Validate, Deserialize)]
pub struct RegisterUser {
    #[validate(
        length(min = 3, message = "must be at least 3 characters"),
        // custom(
        //     function = "validate_unique_username",
        //     arg = "&'v_a AppState",
        //     message = "username already taken"
        // )
    )]
    pub username: String,
    #[validate(
        email(message = "not a valid email address"),
        // custom(
        //     function = "validate_unique_email",
        //     arg = "&'v_a AppState",
        //     message = "email already registered"
        // )
    )]
    pub email: String,
    #[validate(
        length(min = 8, max = 72, message = "must be 8-72 characters"),
        // custom(
        //     function = "validate_password",
        //     message = "password must contain at least one uppercase letter, one lowercase letter, one number, one special character, and be at least 8 characters long"
        // )
    )]
    #[graphql(secret)]
    pub password: String,
}


#[derive(SimpleObject)]
struct User {
    username: String,
    first_name: String,
    last_name: String,
    role: Role,
}

#[derive(InputObject)]
struct UserInput {
    email: String,
    username: String,
    password: String,
    first_name: String,
    last_name: String,
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Enum, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    Master,
    Admin,
    User,
}

// impl Role {
//     pub fn to_i32(&self) -> i32 {
//         match self {
//             Role::Master => 1,
//             Role::Admin => 2,
//             Role::User => 3,
//         }
//     }
// }

#[derive(InputObject)]
struct SignInInput {
    username: String,
    password: String,
}

impl From<&UserEntity> for User {
    fn from(entity: &UserEntity) -> Self {
        let role = match entity.role_id {
            1 => Role::Master,
            2 => Role::Admin,
            _ => Role::User,
        };

        User {
            username: entity.username.clone(),
            first_name: entity.first_name.clone(),
            last_name: entity.last_name.clone(),
            role,
        }
    }
}

struct RoleGuard {
    role: AuthRole,
}

impl RoleGuard {
    fn new(role: AuthRole) -> Self {
        Self { role }
    }
}

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let maybe_getting_role_result = ctx.data_opt::<Result<Option<AuthRole>, CustomError>>();
        match maybe_getting_role_result {
            Some(getting_role_result) => {
                let check_role_result =
                    common_utils::check_user_role_is_allowed(getting_role_result, &self.role);
                match check_role_result {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Error::new(e.message)),
                }
            }
            None => Err(FORBIDDEN_MESSAGE.into()),
        }
    }
}


#[derive(async_graphql::SimpleObject, Debug, Serialize)]
pub struct UserResponse {
    pub user: UserResponseInner,
}

#[derive(async_graphql::SimpleObject, Debug, Serialize)]
pub struct UserResponseInner {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}
