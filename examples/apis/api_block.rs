// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.
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

    let block_by_height = client
        .block_api()
        .get_block_by_height(1422223)
        .await;

    //99123D57F0020054F6B8BB452ED3901B999C85BB21AF1404FEAB981A8CD89F9
    //99123D57F0020054F6B8BB452ED3901B999C85BB21AF1404FEAB981A

    match block_by_height {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{}", err),
    }

    // let blocks_by_height_with_limit = client
    //     .block_api()
    //     .get_blocks_by_height_with_limit(1, 25)
    //     .await;
    //
    // match blocks_by_height_with_limit {
    //     Ok(blocks) => blocks
    //         .iter()
    //         .for_each(|block_info| println!("{}", block_info)),
    //     Err(err) => eprintln!("{}", err),
    // }
    //
    // let block_transactions = client
    //     .block_api()
    //     .get_block_transactions(929413, None, None)
    //     .await;
    //
    // match block_transactions {
    //     Ok(transactions) => transactions
    //         .iter()
    //         .for_each(|tx_info| println!("{}", tx_info)),
    //     Err(err) => eprintln!("{}", err),
    // }
}
