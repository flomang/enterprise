use crate::utils::auth::{Auth};

// Extractors ↓

// #[derive(Debug, Deserialize)]
// pub struct ProfilePath {
//     username: String,
// }

// Client Messages ↓

#[derive(Debug)]
pub struct GetProfile {
    // auth is option in case authentication fails or isn't present
    pub auth: Option<Auth>,
    pub username: String,
}

#[derive(Debug)]
pub struct FollowProfile {
    pub auth: Auth,
    pub username: String,
}

#[derive(Debug)]
pub struct UnfollowProfile {
    pub auth: Auth,
    pub username: String,
}

// JSON response objects ↓

#[derive(Debug, Serialize)]
#[derive(async_graphql::SimpleObject)]
pub struct ProfileResponse {
    pub profile: ProfileResponseInner,
}

#[derive(Debug, Serialize)]
#[derive(async_graphql::SimpleObject)]
pub struct ProfileResponseInner {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}
