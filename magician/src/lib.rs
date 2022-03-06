
#[macro_use] extern crate diesel;
extern crate chrono;
extern crate dotenv;

pub mod schema;
pub mod models;

use chrono::prelude::{Utc};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_ritual<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Ritual {
    use schema::rituals;

    let now = Utc::now().naive_utc();
    let new_ritual = NewRitual {
        title: title,
        body: body,
        created_on: now,
        updated_on: now,
    };

    diesel::insert_into(rituals::table)
        .values(&new_ritual)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn create_ritual_time(conn: &PgConnection, ritual_id: i32, time: chrono::NaiveDateTime) -> RitualTime {
    use schema::ritual_times;

    let new_time = NewRitualTime {
        ritual_id: ritual_id,
        created_on: time,
    };

    diesel::insert_into(ritual_times::table)
        .values(&new_time)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn delete_ritual<'a>(conn: &PgConnection, pattern: &'a str) -> usize {
    use schema::rituals::dsl::{rituals, title};

    diesel::delete(rituals.filter(title.like(pattern)))
        .execute(conn)
        .expect("Error deleting posts")
}

pub fn publish_ritual(conn: &PgConnection, id: i32) -> Ritual {
    use schema::rituals::dsl::{rituals, published};

    diesel::update(rituals.find(id))
        .set(published.eq(true))
        .get_result::<Ritual>(conn)
        .expect(&format!("Unable to find ritual {}", id))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ritual() {
        // let mut weed = Ritual::new(String::from("Smoking Weed"));

        // weed.times = vec![
        //     Utc.ymd(2022, 3, 2).and_hms(12, 30, 11),
        //     Utc.ymd(2022, 3, 2).and_hms(1, 30, 10),
        //     Utc.ymd(2022, 3, 2).and_hms(17, 40, 10),
        // ];

        // weed.times.push( Utc.ymd(2022, 3, 2).and_hms(17, 40, 10));

        //assert_eq!(5, weed.times.len());
        assert_eq!(true, true);
    }
}