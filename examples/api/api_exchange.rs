// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_api::SiriusClient;
use xpx_chain_sdk::account::PublicAccount;

const PUBLIC_KEY: &str = "0A233A17473F77A6DC0FA2B707D70B370B51E7E3C47A9C6D8F74341453121726";

#[tokio::main]
async fn main() {
    let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];

    let sirius_client = SiriusClient::new(node_url).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let network_type = client.network_type();

    let public_account = PublicAccount::from_public_key(PUBLIC_KEY, network_type).unwrap();

    let node_info = client
        .exchange_api()
        .get_account_exchange_info(public_account)
        .await;

    match node_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{}", err),
    }
}
