#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_apis::SiriusClient;

const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";

#[tokio::main]
async fn main() {
    let sirius_client = SiriusClient::new(NODE_URL).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let blockchain_height = client.chain_api().get_blockchain_height().await;
    match blockchain_height {
        Ok(_res) => println!("{}", _res),
        Err(err) => eprintln!("{}", err),
    }

    let blockchain_score = client.chain_api().get_blockchain_score().await;
    match blockchain_score {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{}", err),
    }

    let blockchain_storage = client.chain_api().get_blockchain_storage().await;
    match blockchain_storage {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{}", err),
    }
}
