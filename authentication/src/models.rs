use super::chrono;
use super::schema::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// TODO remove insertable here
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub email_verified: bool,
    pub username: String,
    pub avatar_url: Option<String>,
    pub hash: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: uuid::Uuid,
    pub email: &'a str,
    pub email_verified: bool,
    pub username: &'a str,
    pub hash: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UpdateUserPassword {
    pub id: uuid::Uuid,
    pub hash: String,
    pub updated_at: chrono::NaiveDateTime,
}

impl User {
    pub fn from_details<S: Into<String>, T: Into<String>>(email: S, pwd: T) -> Self {
        let now = chrono::Local::now().naive_local();
        let email = email.into();
        let username = email.clone();

        User {
            id: uuid::Uuid::new_v4(),
            email_verified: false,
            email,
            username,
            hash: pwd.into(),
            avatar_url: None,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "invitations"]
pub struct Invitation {
    pub id: uuid::Uuid,
    pub sender_id: uuid::Uuid,
    pub recipient_email: String,
    pub expires_at: chrono::NaiveDateTime,
}

// any type that implements Into<String> can be used to create Invitation
impl<T> From<T> for Invitation
where
    T: Into<String>,
{
    fn from(email: T) -> Self {
        Invitation {
            id: uuid::Uuid::new_v4(),
            sender_id: uuid::Uuid::nil(),
            recipient_email: email.into(),
            expires_at: chrono::Local::now().naive_local() + chrono::Duration::hours(24),
        }
    }
}

impl Invitation {
    pub fn new(sender_id: uuid::Uuid, recipient_email: String) -> Self {
        Invitation {
            id: uuid::Uuid::new_v4(),
            sender_id: sender_id,
            recipient_email: recipient_email,
            expires_at: chrono::Local::now().naive_local() + chrono::Duration::hours(24),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub avatar_url: Option<String>,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser {
            id: user.id,
            email: user.email,
            username: user.username,
            avatar_url: user.avatar_url,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub message: String,
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn new(message: &str, data: T) -> ResponseBody<T> {
        ResponseBody {
            message: message.to_string(),
            data,
        }
    }
}

#[derive(Serialize)]
pub struct Page<T> {
    pub message: String,
    pub data: Vec<T>,
    pub page_num: i64,
    pub page_size: i64,
    pub total_elements: i64,
}

impl<T> Page<T> {
    pub fn new(
        message: &str,
        data: Vec<T>,
        page_num: i64,
        page_size: i64,
        total_elements: i64,
    ) -> Page<T> {
        Page {
            message: message.to_string(),
            data,
            page_num,
            page_size,
            total_elements,
        }
    }
}