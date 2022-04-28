use futures::{Future, Stream, StreamExt, TryStreamExt};
use coinbase_pro_rs::{WSFeed, CBError, WS_SANDBOX_URL};
use coinbase_pro_rs::structs::wsfeed::*;

#[tokio::main]
async fn main() {
    let stream = WSFeed::connect(WS_SANDBOX_URL,
        &["BTC-USD"], &[ChannelType::Heartbeat]).await.unwrap();

    stream
        .for_each(|msg: Result<Message, CBError>| async {
        match msg.unwrap() {
            Message::Heartbeat {sequence, last_trade_id, time, ..} => println!("{}: seq:{} id{}",
                                                                               time, sequence, last_trade_id),
            Message::Error {message} => println!("Error: {}", message),
            Message::InternalError(_) => panic!("internal_error"),
            other => println!("{:?}", other)
        }
    }).await;
}