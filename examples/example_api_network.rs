/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use xpx_chain_sdk::api::SiriusClient;

#[tokio::main]
async fn main() {
    let node_url = vec!["https://api-2.testnet2.xpxsirius.io:443"];

    let sirius_client = SiriusClient::new_from_urls(&node_url).await;
    let client = match sirius_client {
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
