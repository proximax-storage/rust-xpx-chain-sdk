#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::Client;

use xpx_chain_sdk::apis::sirius_client::SiriusClient;

#[tokio::main]
async fn main() {
    let node = "http://bcstage1.xpxsirius.io:3000";

    let client = SiriusClient::new(node, Client::new());

    let account_info = client.mosaic.get_mosaic(
        "6ad1fa3645ee1987").await;

    match account_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("ERROR: {:?}", err),
    }
}
