#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_apis::SiriusClient;

#[tokio::main]
async fn main() {
    let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];

    let sirius_client = SiriusClient::new(node_url).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let blockchain_height = client
        .chain_api()
        .get_blockchain_height()
        .await;

    match blockchain_height {
        Ok(_res) => println!("{}", _res),
        Err(err) => eprintln!("{}", err),
    }

    let blockchain_score = client
        .chain_api()
        .get_blockchain_score()
        .await;

    match blockchain_score {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{}", err),
    }

    let blockchain_storage = client
        .chain_api()
        .get_blockchain_storage()
        .await;

    match blockchain_storage {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{}", err),
    }
}
