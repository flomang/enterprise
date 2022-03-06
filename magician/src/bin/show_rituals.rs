extern crate magician;
extern crate diesel;

use chrono::prelude::{Utc};
use self::magician::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use magician::schema::rituals::dsl::*;

    let connection = establish_connection();
    let ritual = create_ritual(&connection, "Quit Weed", "Just need a good tbreak!");
    println!("\nSaved draft with id {}", ritual.id);

    let ritual_updated = publish_ritual(&connection, ritual.id);
    println!("{:?}", ritual_updated);

    create_ritual_time(&connection, ritual.id,  Utc::now().naive_utc());
    //let deleted = delete_ritual(&connection, &ritual.title);
    //println!("num deleted: {}", deleted);

    let results = rituals.filter(published.eq(true))
        .limit(5)
        .load::<Ritual>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} rituals", results.len());
    for ritual in results {
        println!("{}", ritual.title);
        println!("----------\n");
        println!("{}", ritual.body);
    }
}