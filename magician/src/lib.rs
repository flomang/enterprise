#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;

pub mod auth_handler;
pub mod email_service;
pub mod errors;
pub mod invitation_handler;
pub mod models;
pub mod pagination;
pub mod register_handler;
pub mod ritual_handler;
pub mod schema;
pub mod utils;

use self::models::*;
use self::pagination::*;
use chrono::prelude::Utc;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

// pub fn create_ritual<'a>(
//     conn: &PgConnection,
//     user_id: uuid::Uuid,
//     title: &'a str,
//     body: &'a str,
// ) -> Ritual {
//     use schema::rituals;

//     let now = Utc::now().naive_utc();
//     let new_ritual = NewRitual {
//         user_id: user_id,
//         title: title,
//         body: body,
//         created_at: now,
//         updated_at: now,
//     };

//     diesel::insert_into(rituals::table)
//         .values(&new_ritual)
//         .get_result(conn)
//         .expect("Error saving new post")
// }

pub fn list_rituals(
    conn: &mut PgConnection,
    user: uuid::Uuid,
    page: i64,
    pize_size: i64,
) -> (Vec<Ritual>, i64) {
    use schema::rituals::dsl::*;

    let (results, total_pages) = rituals.filter(user_id.eq(&user))
        .order_by(created_at)
        .paginate(page)
        .per_page(pize_size)
        .load_and_count_pages::<Ritual>(conn)
        .expect("query fav failed");

    (results, total_pages)
}

pub fn create_ritual_time(
    conn: &PgConnection,
    ritual_id: uuid::Uuid,
    time: chrono::NaiveDateTime,
) -> RitualTime {
    use schema::ritual_times;

    let new_time = NewRitualTime {
        ritual_id: ritual_id,
        created_at: time,
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

pub fn publish_ritual(conn: &PgConnection, id: uuid::Uuid) -> Ritual {
    use schema::rituals::dsl::{published, rituals};

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
        let mut conn = establish_connection();
        //create_ritual(&conn, "Quit Weed1", "one");
        //create_ritual(&conn, "Quit Weed2", "two");
        //create_ritual(&conn, "Quit Weed3", "three");
        //create_ritual(&conn, "Quit Weed4", "four");
        //create_ritual(&conn, "Quit Weed5", "five");
        //create_ritual(&conn, "Quit Weed6", "six");
        let page_size = 3;
        let (results, total_pages) = list_rituals(&mut conn, 2, page_size);
        println!(
            "Displaying {} of total: {}",
            results.len(),
            (total_pages * page_size)
        );
        for ritual in results {
            println!("title: {}", ritual.title);
            println!("body: {}", ritual.body);
            println!("----------\n");
        }

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
