use futures::StreamExt;
//use coinbase_pro_rs::{WSFeed, CBError, WS_SANDBOX_URL, WS_URL};
use clap::Parser;
use coinbase_pro_rs::structs::wsfeed::*;
use coinbase_pro_rs::{CBError, WSFeed, WS_URL};

/// A coinbase pro market feed
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The market to connect to e.g. 'BTC=USD'
    #[arg(short, long)]
    market: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    // TODO args can have multiple markets
    let market = args.market;

    println!("market stream: {}", market);

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
                Message::Ticker(full) => println!(
                    "price: {:?}, best bid: {}, best ask: {}",
                    full.price(),
                    full.bid().unwrap(),
                    full.ask().unwrap()
                ),
                other => println!("{:?}", other),
            }
        })
        .await;
}
