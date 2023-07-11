/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::thread::sleep;
use std::time::Duration;

use xpx_chain_sdk::api::{trace_init, ApiNode};

#[tokio::main]
async fn main() -> Result<(), xpx_chain_sdk::api::error::Error> {
    // let _t = trace_init();

    let client = ApiNode::builder()
        .uri("http://bctestnet3.brimstone.xpxsirius.io:3000")
        // .timeout(Duration::from_secs(1))
        .connect_timeout(Duration::from_secs(5))
        .max_num_retry(10)
        .concurrency_limit(100)
        .build()?
        .connect()
        .await?;

    for i in 0..100 {
        let api = client.network_api();

        tokio::spawn(async move {
            // Get the node information.
            let network_type = api.get_network_type().await;
            match network_type {
                Ok(resp) => println!("{}", resp),
                Err(err) => eprintln!(
                    "Exception when calling network_api->get_get_network_type: {}\n",
                    err
                ),
            }
        });
    }
    sleep(Duration::from_secs(10));

    Ok(())
}
