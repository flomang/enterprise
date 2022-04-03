use serde::Deserialize;

pub mod auth;
pub mod invitation;
pub mod register;

#[derive(Deserialize)]
pub struct PageInfo {
    page: i64,
    page_size: i64,
}