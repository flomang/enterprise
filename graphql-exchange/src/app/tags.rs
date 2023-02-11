// Client Messages ↓

#[derive(Debug)]
pub struct GetTags {}

// JSON response objects ↓

#[derive(async_graphql::SimpleObject)]
#[derive(Serialize)]
pub struct TagsResponse {
    pub tags: Vec<String>,
}
