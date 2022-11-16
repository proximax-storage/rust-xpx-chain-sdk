/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */


use xpx_chain_sdk::api::{Client, Endpoint};

#[tokio::main]
async fn main() {
    let ep1 = Endpoint::from_shared("http://api-2.testnet2.xpxsirius.io:3000").unwrap();

    let client = ep1.connect().await;
    let client = match client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
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
