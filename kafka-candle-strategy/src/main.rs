use dotenv::dotenv;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;
use std::env;
use log::{info, error};


// 60 seconds for candle interval
static CANDLE_INTERVAL: i64 = 60;

fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let broker = env::var("KAFKA_BROKER").expect("KAFKA_BROKER must be set");
    let topic = env::var("KAFKA_TOPIC").expect("KAFKA_TOPIC must be set");
    let group = env::var("KAFKA_GROUP").expect("KAFKA_GROUP is not set");

    if let Err(e) = consume_messages(group, topic, vec![broker]) {
        error!("Failed consuming messages: {}", e);
    }
}

use chrono::{DateTime, Utc};
use coinbase_pro_rs::structs::wsfeed::Ticker;
use std::collections::BTreeMap;
use bigdecimal::{BigDecimal, FromPrimitive};

#[derive(Debug)]
struct Candle {
    #[allow(unused)]
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: BigDecimal,
    #[allow(unused)]
    time: DateTime<Utc>,
}

impl Candle {
    fn new(time: DateTime<Utc>, price: f64) -> Self {
        Self {
            open: price,
            high: price,
            low: price,
            close: price,
            volume: BigDecimal::from_f64(0.0).unwrap(),
            time,
        }
    }
}

impl std::fmt::Display for Candle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} -- O: {} H: {} L: {} C: {} V: {:.8}", self.time, self.open, self.high, self.low, self.close, self.volume)
    }
}

fn consume_messages(group: String, topic: String, brokers: Vec<String>) -> Result<(), KafkaError> {
    let mut con = Consumer::from_hosts(brokers)
        .with_topic(topic)
        .with_group(group)
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(Some(GroupOffsetStorage::Kafka))
        .create()?;

    // Create a BTreeMap to store OHLC candles, where the key is the candle start time
    // we use a BTreeMap because it keeps the keys sorted
    let mut ohlc_candles: BTreeMap<DateTime<Utc>, Candle> = BTreeMap::new();

    loop {
        let mss = con.poll()?;

        for ms in mss.iter() {
            for m in ms.messages() {
                let str = String::from_utf8(m.value.to_vec()).unwrap();
                let trade: Ticker = serde_json::from_str(&str).unwrap();

                if let Ticker::Full {
                    trade_id: _,
                    sequence: _,
                    time,
                    product_id: _,
                    price,
                    side: _,
                    last_size,
                    best_bid: _,
                    best_ask: _,
                } = trade
                {
                    // Get the candle start time based on the candle_interval
                    let candle_start_time = time.timestamp() / CANDLE_INTERVAL * CANDLE_INTERVAL;

                    // Get or insert the OHLC candle for the current interval
                    let dt = DateTime::<Utc>::from_timestamp(candle_start_time, 0)
                        .expect("invalid timestamp");

                    let len = ohlc_candles.len();

                    // scope to limit mutable borrow
                    {
                        let candle_entry = ohlc_candles
                            .entry(dt)
                            .or_insert(Candle::new(dt, price));

                        // Update OHLC values
                        if price > candle_entry.high {
                            candle_entry.high = price;
                        }
                        if price < candle_entry.low {
                            candle_entry.low = price;
                        }

                        candle_entry.close = price;

                        // using BigDecimal to avoid floating point errors
                        candle_entry.volume += BigDecimal::from_f64(last_size).unwrap();
                    }

                    // if we're working on a new candle then print the previous candle 
                    if len < ohlc_candles.len() && len > 0 {
                        let sorted: Vec<&DateTime<Utc>> = ohlc_candles.keys().collect();

                        let previous_candle = ohlc_candles.get(sorted[sorted.len()-2]).unwrap();
                        // log previous candle as it should be finished
                        info!("{}", previous_candle);

                    }
                }
            }
            let _ = con.consume_messageset(ms);
        }
        con.commit_consumed()?;
    }
}
