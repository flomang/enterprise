//use grin_api::ForeignRpc;
use easy_jsonrpc_mw::{BoundMethod, Response};
use grin_api::foreign_rpc::foreign_rpc;
use grin_pool::types::{PoolEntry};
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::{thread, time};
use log::{info};

// Demonstrate an example JSON-RCP call against grin.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3413);
    let result_version = rpc(&server_addr, &foreign_rpc::get_version().unwrap()).await;
    info!("version: {:?}", result_version);

    let result_tip = rpc(&server_addr, &foreign_rpc::get_tip().unwrap()).await;
    info!("tip: {:?}", result_tip);

    let delay = time::Duration::from_secs(1);
    let mut all_txns: Vec<PoolEntry> = vec![];

    while let Ok(result) = rpc(&server_addr, &foreign_rpc::get_unconfirmed_transactions().unwrap()).await {
        let result_tip = rpc(&server_addr, &foreign_rpc::get_tip().unwrap()).await;
        info!("tip: {:?}", result_tip);

        if let Ok(txns) = result {
             if all_txns.len() != txns.len() {
                 all_txns = txns;

                 //let result = rpc(&server_addr, &foreign_rpc::get_pool_size().unwrap());
                 //println!("size: {:?}", result);
                 for txn in all_txns.iter() {
                    let inputs = txn.tx.body.inputs.len();
                    let outputs = txn.tx.body.outputs.len();
                    let kernels = txn.tx.body.kernels.len();

                    info!("----");
                    info!("\t at: {}", txn.tx_at);
                    info!("\t src: {:?}", txn.src);
                    info!("\t kernels: {:?}", kernels);
                    info!("\t inputs: {:?}", inputs);
                    info!("\t outputs: {:?}", outputs);
                    info!("\t tx: {:?}", txn.tx);
                 }
             }
        } else {
            println!("nope")
        }
        thread::sleep(delay);
    }

    Ok(())
}


async fn rpc<R: Deserialize<'static>>(
    addr: &SocketAddr,
    method: &BoundMethod<'_, R>,
) -> Result<R, RpcErr> {
    let (request, tracker) = method.call();
    let json_response = post(addr, &request.as_request()).await?;
    let mut response = Response::from_json_response(json_response)?;
    Ok(tracker.get_return(&mut response)?)
}

async fn post(addr: &SocketAddr, body: &Value) -> Result<Value, reqwest::Error> {
    let client = Client::new();
    let response = client
        .post(&format!("http://{}/v2/foreign", addr))
        .json(body)
        .send()
        .await?;

    let thing = response.error_for_status()?
    .json::<Value>().await?;
    Ok(thing)
}

#[derive(Debug)]
enum RpcErr {
    Http(reqwest::Error),
    InvalidResponse,
}

impl From<easy_jsonrpc_mw::InvalidResponse> for RpcErr {
    fn from(_other: easy_jsonrpc_mw::InvalidResponse) -> Self {
        RpcErr::InvalidResponse
    }
}

impl From<easy_jsonrpc_mw::ResponseFail> for RpcErr {
    fn from(_other: easy_jsonrpc_mw::ResponseFail) -> Self {
        RpcErr::InvalidResponse
    }
}

impl From<reqwest::Error> for RpcErr {
    fn from(other: reqwest::Error) -> Self {
        RpcErr::Http(other)
    }
}
