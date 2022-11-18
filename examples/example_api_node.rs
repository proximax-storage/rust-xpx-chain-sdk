/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use xpx_chain_sdk::api::{Endpoint};

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

    let node_api = client.node_api();

    // Get the node information.
    let node_info = node_api.get_node_info().await;
    match node_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("Exception when calling node_api->get_node_info: {}\n", err),
    }

    // // Get the node time.
    // let node_time = node_api.get_node_time().await;
    // match node_time {
    //     Ok(resp) => println!("{}", resp),
    //     Err(err) => eprintln!("Exception when calling node_api->get_node_time: {}\n", err),
    // }
    //
    // // Get the node time.
    // let node_peers = node_api.get_node_peers().await;
    // match node_peers {
    //     Ok(resp) => println!("{:?}", resp),
    //     Err(err) => eprintln!("Exception when calling node_api->get_node_peers: {}\n", err),
    // }
}
