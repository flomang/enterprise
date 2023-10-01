use futures::StreamExt;
//use coinbase_pro_rs::{WSFeed, CBError, WS_SANDBOX_URL, WS_URL};
use clap::Parser;
use coinbase_pro_rs::structs::wsfeed::*;
use coinbase_pro_rs::{CBError, WSFeed, WS_URL};

use std::time::Duration;

use kafka::error::Error as KafkaError;
use kafka::producer::{Producer, Record, RequiredAcks};

/// A coinbase pro market feed kafka producer
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The market to connect to e.g. 'BTC=USD'
    #[arg(short, long)]
    market: String,
    /// Kafka broker defaults to 'localhost:9092' 
    #[arg(short, long, default_value = "localhost:9092")]
    broker: String,
    /// Kafka topic 
    #[arg(short, long)]
    topic: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    // TODO args can have multiple markets
    let market = args.market;
    let broker = args.broker;
    let topic = args.topic;

    println!("market stream: {}, broker: {}, topic: {}", market, broker,topic);

    let stream = WSFeed::connect(WS_URL, &[&market], &[ChannelType::Ticker])
        .await
        .unwrap();

    stream
        .for_each(|msg: Result<Message, CBError>| async {
            match msg.unwrap() {
                Message::Heartbeat {
                    sequence,
                    last_trade_id,
                    time,
                    ..
                } => println!("{}: seq:{} id{}", time, sequence, last_trade_id),
                Message::Error { message } => println!("Error: {}", message),
                Message::InternalError(_) => panic!("internal_error"),
                Message::Ticker(full) => {
                    let data = serde_json::to_string(&full).unwrap();

                    // produce kafka messaage
                    match produce_message(data.as_bytes(), &topic, vec![broker.to_owned()]) {
                        Ok(_) => println!("{:?}", data),
                        Err(e) => println!("Failed producing messages: {}", e),
                    }
                }

                other => println!("{:?}", other),
            }
        })
        .await;
}

fn produce_message<'a, 'b>(
    data: &'a [u8],
    topic: &'b str,
    brokers: Vec<String>,
) -> Result<(), KafkaError> {
    // ~ create a producer. this is a relatively costly operation, so
    // you'll do this typically once in your application and re-use
    // the instance many times.
    let mut producer = Producer::from_hosts(brokers)
        // ~ give the brokers one second time to ack the message
        .with_ack_timeout(Duration::from_secs(1))
        // ~ require only one broker to ack the message
        .with_required_acks(RequiredAcks::One)
        // ~ build the producer with the above settings
        .create()?;

    // ~ now send a single message.  this is a synchronous/blocking
    // operation.

    // ~ we're sending 'data' as a 'value'. there will be no key
    // associated with the sent message.

    // ~ we leave the partition "unspecified" - this is a negative
    // partition - which causes the producer to find out one on its
    // own using its underlying partitioner.
    producer.send(&Record {
        topic,
        partition: -1,
        key: (),
        value: data,
    })?;

    // ~ we can achieve exactly the same as above in a shorter way with
    // the following call
    producer.send(&Record::from_value(topic, data))?;

    Ok(())
}
