#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::Client;

use xpx_chain_sdk::apis::sirius_client::SiriusClient;

#[tokio::main]
async fn main() {
    let node = "http://bctestnetswap.xpxsirius.io:3000";

    let client = SiriusClient::new(node, Client::new());

    let block_by_height = client.clone().block.get_block_by_height(1).await;
    match block_by_height {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{:?}", err),
    }

    let blocks_by_height_with_limit = client.block.get_blocks_by_height_with_limit(1, 25).await;
    match blocks_by_height_with_limit {
        Ok(resp) => {
            for i in resp {
                println!("{}", i)
            }
        }
        Err(err) => eprintln!("{:?}", err),
    }
}
