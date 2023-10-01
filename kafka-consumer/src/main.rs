use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;
use tracing_subscriber;
use dotenv::dotenv;
use std::env;

/// This program demonstrates consuming messages through a `Consumer`.
/// This is a convenient client that will fit most use cases.  Note
/// that messages must be marked and committed as consumed to ensure
/// only once delivery.
fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let broker = env::var("KAFKA_BROKER").expect("KAFKA_BROKER must be set");
    let topic = env::var("KAFKA_TOPIC").expect("KAFKA_TOPIC must be set");
    let group = env::var("KAFKA_GROUP").expect("KAFKA_GROUP is not set");

    if let Err(e) = consume_messages(group, topic, vec![broker]) {
        println!("Failed consuming messages: {}", e);
    }
}

fn consume_messages(group: String, topic: String, brokers: Vec<String>) -> Result<(), KafkaError> {
    let mut con = Consumer::from_hosts(brokers)
        .with_topic(topic)
        .with_group(group)
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(Some(GroupOffsetStorage::Kafka))
        .create()?;

    loop {
        let mss = con.poll()?;

        for ms in mss.iter() {
            for m in ms.messages() {
                println!("{:?}", String::from_utf8(m.value.to_vec()).unwrap());
            }
            let _ = con.consume_messageset(ms);
        }
        con.commit_consumed()?;
    }
}