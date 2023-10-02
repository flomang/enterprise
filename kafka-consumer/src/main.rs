use dotenv::dotenv;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;
use std::env;
use tracing_subscriber;

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

use chrono::{DateTime, Utc};
use coinbase_pro_rs::structs::wsfeed::Ticker;
use std::collections::HashMap;
// use ta::indicators::ExponentialMovingAverage;
// use ta::Next;

#[derive(Debug)]
struct Candle {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    time: DateTime<Utc>,
}

impl Candle {
    fn new(time: DateTime<Utc>, price: f64, volume: f64) -> Self {
        Self {
            open: price,
            high: price,
            low: price,
            close: price,
            volume,
            time,
        }
    }
}

fn consume_messages(group: String, topic: String, brokers: Vec<String>) -> Result<(), KafkaError> {
    let mut con = Consumer::from_hosts(brokers)
        .with_topic(topic)
        .with_group(group)
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(Some(GroupOffsetStorage::Kafka))
        .create()?;

    // Define the candle interval in seconds (e.g., 60 seconds for 1 minute candles)
    let candle_interval = 60;

    // Create a HashMap to store OHLC candles, where the key is the candle start time
    //let mut ohlc_candles: HashMap<DateTime<Utc>, (f64, f64, f64, f64)> = HashMap::new();
    let mut ohlc_candles: HashMap<DateTime<Utc>, Candle> = HashMap::new();

    loop {
        let mss = con.poll()?;

        for ms in mss.iter() {
            for m in ms.messages() {
                let str = String::from_utf8(m.value.to_vec()).unwrap();
                let trade: Ticker = serde_json::from_str(&str).unwrap();

                match trade {
                    Ticker::Full {
                        trade_id: _,
                        sequence: _,
                        time,
                        product_id: _,
                        price,
                        side: _,
                        last_size,
                        best_bid: _,
                        best_ask: _,
                    } => {
                        //println!("Trade ID: {}", trade_id);
                        // Get the candle start time based on the candle_interval
                        let candle_start_time =
                            time.timestamp() / candle_interval * candle_interval;

                        // Get or insert the OHLC candle for the current interval
                        let dt = DateTime::<Utc>::from_timestamp(candle_start_time, 0)
                            .expect("invalid timestamp");

                        let candle_entry = ohlc_candles
                            .entry(dt)
                            .or_insert(Candle::new(dt, price, last_size));

                        // Update OHLC values
                        if price > candle_entry.high {
                            candle_entry.high = price;
                        }
                        if price < candle_entry.low {
                            candle_entry.low = price;
                        }

                        candle_entry.close = price;
                        candle_entry.volume += last_size;

                        println!("{:?}", candle_entry);
                        // Access other fields as needed
                    }
                    _ => {
                        // Handle the Ticker::Empty variant if needed
                    }
                }
            }
            let _ = con.consume_messageset(ms);
        }
        con.commit_consumed()?;
    }
}
