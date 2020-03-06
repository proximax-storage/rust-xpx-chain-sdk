#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::Client;

use xpx_chain_sdk::sirius_client::SiriusClient;

#[tokio::main]
async fn main() {
    let node = "http://bctestnet1.brimstone.xpxsirius.io:3000";

    let client = SiriusClient::new(node, Client::new());

    let node_info = client.clone().node.get_node_info().await;
    match node_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{:?}", err),
    }

    let node_time = client.node.get_node_time().await;
    match node_time {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{:?}", err),
    }
}
