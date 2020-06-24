// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_api::SiriusClient;

const HASH_ONE: &str = "2C0550F4F8A9D6809C39B06E8B89750211256893795E5F26499B48716DE3F7DE";
const HASH_TWO: &str = "5EC5C0E766B3DF81FBAD0E4FD794828002763905FEDC47208520E90FBED783B4";

#[tokio::main]
async fn main() {
    let node_url = vec!["http://bctestnet3.brimstone.xpxsirius.io:3000"];

    let sirius_client = SiriusClient::new(node_url).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let transaction = client.transaction_api().get_transaction(HASH_ONE).await;

    match transaction {
        Ok(tx) => println!("{}", tx),
        Err(err) => eprintln!("{}", err),
    }

    let transaction_status = client
        .transaction_api()
        .get_transaction_status(HASH_ONE)
        .await;

    match transaction_status {
        Ok(status) => println!("{}", status),
        Err(err) => eprintln!("{}", err),
    }

    let transactions_statuses = client
        .transaction_api()
        .get_transactions_statuses(vec![HASH_ONE, HASH_TWO])
        .await;

    match transactions_statuses {
        Ok(statuses) => statuses.iter().for_each(|status| println!("{}", status)),
        Err(err) => eprintln!("{}", err),
    }

    let transaction = client.transaction_api().get_transaction(HASH_ONE).await;

    match transaction {
        Ok(tx) => println!("{}", tx),
        Err(err) => eprintln!("{}", err),
    }

    let transactions = client
        .transaction_api()
        .get_transactions(vec![HASH_ONE, HASH_TWO])
        .await;

    match transactions {
        Ok(txs) => txs.into_iter().for_each(|tx_info| println!("{}", tx_info)),
        Err(err) => eprintln!("{}", err),
    }
}
