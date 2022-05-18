use crate::{
    config::db::Pool,
};
use library::auth::Claims;
use actix_web::web;
use jsonwebtoken::{DecodingKey, TokenData, Validation};

pub static KEY: [u8; 16] = *include_bytes!("../secret.key");

pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
    jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(&KEY),
        &Validation::default(),
    )
}

pub fn verify_token(
    token_data: &TokenData<Claims>,
    _pool: &web::Data<Pool>,
) -> Result<String, String> {
    Ok(token_data.claims.username.to_string())
    //if User::is_valid_login_session(&token_data.claims, &pool.get().unwrap()) {
    //    Ok(token_data.claims.user.to_string())
    //} else {
    //    Err("Invalid token".to_string())
    //}
}
