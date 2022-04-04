use actix_identity::{CookieIdentityPolicy, IdentityService};
use time::Duration;

pub fn cookie_policy(domain: String, duration: Duration) -> IdentityService<CookieIdentityPolicy>{
    IdentityService::new(
        CookieIdentityPolicy::new(super::SECRET_KEY.as_bytes())
            .name("auth")
            .path("/")
            .domain(domain)
            .max_age(duration) // one day in seconds
            .secure(false), // this can only be true if you have https
    )
}