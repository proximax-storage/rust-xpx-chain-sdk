/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */


use xpx_chain_sdk::api::{Client, Endpoint};

#[tokio::main]
async fn main() {
    let client = match Endpoint::from_static("http://api-1.testnet2.xpxsirius.io:3000")
        .connect()
        .await
    {
        Ok(client) => client,
        Err(err) => panic!(
            "Exception when calling endpoint connect{}\n",
            err
        ),
    };

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
}
