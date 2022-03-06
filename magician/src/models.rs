use super::schema::{rituals, ritual_times};
use super::chrono;


#[derive(Debug, Queryable)]
pub struct Ritual {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created_on: chrono::NaiveDateTime,
    pub updated_on: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="rituals"]
pub struct NewRitual<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub created_on: chrono::NaiveDateTime,
    pub updated_on: chrono::NaiveDateTime,
}

#[derive(Debug, Queryable)]
pub struct RitualTime {
    pub id: i32,
    pub ritual_id: i32,
    pub created_on: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="ritual_times"]
pub struct NewRitualTime {
    pub ritual_id: i32,
    pub created_on: chrono::NaiveDateTime,
}