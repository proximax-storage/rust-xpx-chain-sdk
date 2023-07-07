/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::time::Duration;

use xpx_chain_sdk::api::{trace_init, ApiNode};

#[tokio::main]
async fn main() -> Result<(), xpx_chain_sdk::api::error::Error> {
    let _t = trace_init();

    let client = ApiNode::builder()
        .uri("http://bctestnet3.brimstone.xpxsirius.io:300")
        // .timeout(Duration::from_secs(1))
        .connect_timeout(Duration::from_secs(1))
        .max_num_retry(10)
        .build()?
        .connect()
        .await?;

    let network_api = client.network_api();

    // Get the node information.
    let network_type = network_api.get_network_type().await;
    match network_type {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!(
            "Exception when calling network_api->get_get_network_type: {}\n",
            err
        ),
    }

    Ok(())
}
