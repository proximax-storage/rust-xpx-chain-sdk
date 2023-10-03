/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::time::Duration;

use xpx_chain_sdk::api::ApiNode;

#[tokio::main]
async fn main() -> Result<(), xpx_chain_sdk::api::error::Error> {
    let client = ApiNode::builder()
        .uri("http://bctestnet3.brimstone.xpxsirius.io:3000")
        // .timeout(Duration::from_secs(1))
        .connect_timeout(Duration::from_secs(5))
        .build()?
        .connect()
        .await?;

    let chain_api = client.chain_api();

    // Get the current `Height` of the chain.
    let block_height = chain_api.get_block_height().await;
    match block_height {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("Exception when calling chain_api->get_block_height: {}\n", err),
    }

    // Gets a block from the chain that has the given height.
    let block_by_height = chain_api.get_block_by_height(1).await;
    match block_by_height {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("Exception when calling chain_api->get_block_by_height: {}\n", err),
    }

    // Get the current score of the chain.
    let block_score = chain_api.get_block_score().await;
    match block_score {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("Exception when calling chain_api->get_block_score: {}\n", err),
    }

    // Get the storage information of the node.
    let block_storage = chain_api.get_block_storage().await;
    match block_storage {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("Exception when calling chain_api->get_block_storage: {}\n", err),
    }

    // Gets up to limit number of blocks after given block height.
    let block = chain_api.get_blocks_by_height_with_limit(1, 25).await;
    match block {
        Ok(resp) => println!("{:#?}", resp),
        Err(err) => eprintln!(
            "Exception when calling chain_api->get_blocks_by_height_with_limit: {}\n",
            err
        ),
    };
    Ok(())
}
