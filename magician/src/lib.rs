
#[macro_use] extern crate diesel;
extern crate chrono;
extern crate dotenv;

use chrono::prelude::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

struct Ritual {
    name: String,
    times: Vec<DateTime<Utc>>,
}

impl Ritual {
    pub fn new(name: String) -> Ritual {
        Ritual { name, times: Vec::new() }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ritual() {
        let mut weed = Ritual::new(String::from("Smoking Weed"));

        weed.times = vec![
            Utc.ymd(2022, 3, 2).and_hms(12, 30, 11),
            Utc.ymd(2022, 3, 2).and_hms(1, 30, 10),
            Utc.ymd(2022, 3, 2).and_hms(17, 40, 10),
        ];

        weed.times.push( Utc.ymd(2022, 3, 2).and_hms(17, 40, 10));

        assert_eq!(5, weed.times.len());
    }
}