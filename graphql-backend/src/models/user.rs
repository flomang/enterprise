use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::db::schema::users;

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

