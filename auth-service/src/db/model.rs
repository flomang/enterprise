use diesel::prelude::*;
use uuid::Uuid;
use super::schema::users;

#[derive(Identifiable, Queryable)]
#[diesel(table_name = users)]
pub struct UserEntity {
    pub id: Uuid,
    pub role_id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub email_verified: bool,
    pub hash: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUserEntity {
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub hash: String,
    pub role_id: i32,
}

use chrono::NaiveDateTime;

#[derive(Debug, Queryable, Identifiable)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub email_verified: bool,
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub hash: String,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserChange {
    pub username: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub hash: Option<String>,
}
