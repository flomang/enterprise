use std::str::FromStr;

use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::db::schema::users;

// Diesel models for users table ↓
#[derive(Debug, Queryable, Identifiable)]
pub struct User {
    pub id: Uuid,
    pub role_id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub email_verified: bool,
    pub hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn token_response(&self) -> UserResponse {
        UserResponse {
            user: UserResponseInner {
                token: Some(self.generate_jwt().unwrap()),
                email: self.email.clone(),
                username: self.username.clone(),
                first_name: self.first_name.clone(),
                last_name: self.last_name.clone(),
            },
        }
    }

    pub fn non_token_response(&self) -> UserResponse {
        UserResponse {
            user: UserResponseInner {
                token: None,
                email: self.email.clone(),
                username: self.username.clone(),
                first_name: self.first_name.clone(),
                last_name: self.last_name.clone(),
            },
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub hash: String,
    pub role_id: i32,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserChange {
    pub username: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub hash: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserRoleChange {
    pub role_id: i32,
}


// GraphQL Client Messages ↓
use super::auth::Auth;
use crate::graphql::AppState;
use crate::utils::jwt::CanGenerateJwt;
use regex::Regex;
use validator::{Validate, ValidationError};

lazy_static! {
    static ref RE_USERNAME: Regex = Regex::new(r"^[_0-9a-zA-Z]+$").unwrap();
}

// this is a graphql input object for register user
// it uses validate to validate the input
#[derive(async_graphql::InputObject, Debug, Validate, Deserialize)]
pub struct RegisterUser {
    #[validate(
        length(min = 3, message = "must be at least 3 characters"),
        custom(
            function = "validate_unique_username",
            arg = "&'v_a AppState",
            message = "username already taken"
        )
    )]
    pub username: String,
    #[validate(
        email(message = "not a valid email address"),
        custom(
            function = "validate_unique_email",
            arg = "&'v_a AppState",
            message = "email already registered"
        )
    )]
    pub email: String,
    #[validate(
        length(min = 8, max = 72, message = "must be 8-72 characters"),
        custom(
            function = "validate_password",
            message = "password must contain at least one uppercase letter, one lowercase letter, one number, one special character, and be at least 8 characters long"
        )
    )]
    #[graphql(secret)]
    pub password: String,
    #[validate(length(min = 1, message = "cannot be empty"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "cannot be empty"))]
    pub last_name: String,
}

// validate unique username
fn validate_unique_username(username: &str, state: &AppState) -> Result<(), ValidationError> {
    let result = async_std::task::block_on(state.db.send(FindUser {
        username: username.trim().to_string(),
    }))
    .unwrap();

    match result {
        Ok(_) => Err(ValidationError::new("invalid_username")),
        Err(_) => Ok(()),
    }
}

// validate unique email
fn validate_unique_email(email: &str, state: &AppState) -> Result<(), ValidationError> {
    let result = async_std::task::block_on(state.db.send(FindEmail {
        email: email.trim().to_string(),
    }))
    .unwrap();

    match result {
        Ok(_) => Err(ValidationError::new("invalid_email")),
        Err(_) => Ok(()),
    }
}

// validate an email address
fn validate_email_exists(email: &str, state: &AppState) -> Result<(), ValidationError> {
    let result = async_std::task::block_on(state.db.send(FindEmail {
        email: email.trim().to_string(),
    }))
    .unwrap();

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("invalid_email")),
    }
}

fn validate_username_exists(username: &str, state: &AppState) -> Result<(), ValidationError> {
    let result = async_std::task::block_on(state.db.send(FindUser {
        username: username.trim().to_string(),
    }))
    .unwrap();

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("invalid_username")),
    }
   
}

// Validate password strength. Must contain:
// * at least one uppercase letter
// * one lowercase letter
// * one number
// * one special character
fn validate_password(password: &str) -> Result<(), ValidationError> {
    // and special characters
    if password.chars().any(char::is_uppercase)
        && password.chars().any(char::is_lowercase)
        && password.chars().any(char::is_numeric)
        && password.chars().any(|c| !c.is_alphanumeric())
    {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_password"))
    }
}

fn validate_role(role: &str) -> Result<(), ValidationError> {
    match Role::from_str(&role.to_ascii_uppercase()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("invalid_role")),
    }
}

#[derive(async_graphql::InputObject, Debug, Validate, Deserialize)]
pub struct LoginUser {
    #[validate(email(message = "not a valid email address"))]
    pub email: String,
    #[graphql(secret)]
    pub password: String,
}

pub struct FindUser {
    pub username: String,
}

pub struct FindEmail {
    pub email: String,
}

#[derive(async_graphql::InputObject, Debug, Validate, Deserialize)]
pub struct ForgotPassword {
    #[validate(
        email(message = "not a valid email address"),
        custom(
            function = "validate_email_exists",
            arg = "&'v_a AppState",
            message = "no account matches this email address"
        )
    )]
    pub email: String,
}

#[derive(async_graphql::InputObject, Debug, Validate, Deserialize)]
pub struct UpdateUser {
    #[validate(
        length(min = 3, message = "must be at least 3 characters long"),
        custom(
            function = "validate_unique_username",
            arg = "&'v_a AppState",
            message = "username already taken"
        )
    )]
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    #[validate(length(min = 8, max = 72, message = "must be 8-72 characters long"))]
    pub password: Option<String>,
    #[validate(length(min = 1, message = "cannot be empty"))]
    pub first_name: Option<String>,
    #[validate(length(min = 1, message = "cannot be empty"))]
    pub last_name: Option<String>,
}

#[derive(async_graphql::InputObject, Debug, Validate, Deserialize)]
pub struct UpdateUserRole {
    #[validate(
        custom(
            function = "validate_username_exists",
            arg = "&'v_a AppState",
            message = "user not found"
        )
    )]
    pub username: String,
    #[validate(
        custom(
            function = "validate_role",
            message = "param must be 'admin' or 'user'"
        )
    )]
    pub role: String,
}

#[derive(Debug)]
pub struct UpdateUserOuter {
    pub auth: Auth,
    pub update_user: UpdateUser,
}

// JSON response objects ↓
#[derive(async_graphql::SimpleObject, Debug, Serialize)]
pub struct UserResponse {
    pub user: UserResponseInner,
}

#[derive(async_graphql::SimpleObject, Debug, Serialize)]
pub struct UserResponseInner {
    pub email: String,
    pub token: Option<String>,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

impl UserResponse {
    pub fn from_auth(auth: Auth) -> Self {
        UserResponse {
            user: UserResponseInner {
                token: Some(auth.token),
                email: auth.user.email,
                username: auth.user.username,
                first_name: auth.user.first_name,
                last_name: auth.user.last_name,
            },
        }
    }
}

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
