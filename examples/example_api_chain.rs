#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::Client;

use xpx_chain_sdk::apis::sirius_client::SiriusClient;

#[tokio::main]
async fn main() {
    let node = "http://bctestnetswap.xpxsirius.io:3000";

    let client = SiriusClient::new(node, Client::new());

    let blockchain_height = client.clone().chain.get_blockchain_height().await;
    match blockchain_height {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{:?}", err),
    }

    let blockchain_score = client.clone().chain.get_blockchain_score().await;
    match blockchain_score {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{:?}", err),
    }

    let blockchain_storage = client.clone().chain.get_blockchain_storage().await;
    match blockchain_storage {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{:?}", err),
    }
}
