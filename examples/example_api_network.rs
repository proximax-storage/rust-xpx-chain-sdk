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
        .max_num_retry(5)
        .concurrency_limit(100)
        .build()?
        .connect()
        .await?;

    let api = client.node_api();

    // Get the node information.
    let network_type = api.get_node_info().await;
    match network_type {
        Ok(resp) => println!("{}", resp.friendly_name),
        Err(err) => {
            eprintln!("Exception when calling network_api->get_get_network_type: {}\n", err)
        }
    }

    Ok(())
}
