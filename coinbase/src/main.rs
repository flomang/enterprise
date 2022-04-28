use futures::StreamExt;
use coinbase_pro_rs::{WSFeed, CBError, WS_SANDBOX_URL, WS_URL};
use coinbase_pro_rs::structs::wsfeed::*;

#[tokio::main]
async fn main() {
    let stream = WSFeed::connect(WS_URL,
        &["BTC-USD"], &[ChannelType::Ticker]).await.unwrap();

    stream
        .for_each(|msg: Result<Message, CBError>| async {
        match msg.unwrap() {
            Message::Heartbeat {sequence, last_trade_id, time, ..} => println!("{}: seq:{} id{}",
                                                                               time, sequence, last_trade_id),
            Message::Error {message} => println!("Error: {}", message),
            Message::InternalError(_) => panic!("internal_error"),
            Message::Ticker( full) => println!("price: {:?}, best bid: {}, best ask: {}", full.price(), full.bid().unwrap(), full.ask().unwrap()),
            other => println!("{:?}", other)
        }
    }).await;
}