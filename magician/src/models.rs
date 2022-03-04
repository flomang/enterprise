#[derive(diesel::Queryable)]
pub struct Ritual {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}