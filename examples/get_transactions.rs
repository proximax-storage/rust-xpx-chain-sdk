#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_sdk::apis::sirius_client::SiriusClient;
use hyper::Client;

#[tokio::main]
async fn main() {

    let node = "http://bctestnetswap.xpxsirius.io:3000";

    let client = SiriusClient::new(node, Client::new());

    let status = client.transaction.get_transaction_status(
        "5BD1AE427B10E50902DE8037B7A551104727732B2C97A08A3DD19342714337A5"
    ).await;

    match status {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("RESP_ERROR: {:?}", err),
    }
}
