#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::Client;

use xpx_chain_sdk::sirius_client::SiriusClient;

const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";

#[tokio::main]
async fn main() {
    let sirius_client = SiriusClient::new(NODE_URL, Client::new()).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let block_by_height = client.to_owned().block.get_block_by_height(1).await;
    match block_by_height {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{}", err),
    }

    let blocks_by_height_with_limit = client.to_owned().block.get_blocks_by_height_with_limit(1, 25).await;
    match blocks_by_height_with_limit {
        Ok(blocks) => {
            blocks.iter().for_each(|block_info| {
                println!("{}", block_info)
            })
        }
        Err(err) => eprintln!("{}", err),
    }

    let block_transactions = client.block.get_block_transactions(929413, None, None).await;
    match block_transactions {
        Ok(transactions) => {
            transactions.iter().for_each(|tx_info| {
                println!("{}", tx_info)
            })
        }
        Err(err) => eprintln!("{}", err),
    }
}
