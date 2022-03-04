use super::schema::rituals;

#[derive(Queryable)]
pub struct Ritual {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name="rituals"]
pub struct NewRitual<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
