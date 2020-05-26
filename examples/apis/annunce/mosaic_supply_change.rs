// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_apis::SiriusClient;
use xpx_chain_sdk::account::Account;
use xpx_chain_sdk::mosaic::{MosaicId, MosaicSupplyType};
use xpx_chain_sdk::transaction::{Deadline, MosaicSupplyChangeTransaction};
use xpx_chain_sdk::Uint64;

const PRIVATE_KEY: &str = "7D3E959EB0CD69CC1DB6E9C62CB81EC52747AB56FA740CF18AACB5003429AD2E";

#[tokio::main]
async fn main() {
    let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];

    let sirius_client = SiriusClient::new(node_url).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let generation_hash = client.generation_hash();

    // let network_type = xpx_chain_sdk::network::PUBLIC_TEST;
    let network_type = client.network_type();

    // Deadline default 1 hour
    // let deadline = Deadline::new(1, 0, 0);
    let deadline = Deadline::default();

    let account = Account::from_private_key(PRIVATE_KEY, network_type).unwrap();

    let mosaic_supply = MosaicSupplyChangeTransaction::new(
        deadline,
        MosaicSupplyType::Increase,
        MosaicId::from_hex("389B57CFE6FB5394").unwrap(),
        Uint64::new(100000),
        network_type,
    );

    if let Err(err) = &mosaic_supply {
        panic!("{}", err)
    }

    let sig_mosaic_supply = account.sign(mosaic_supply.unwrap(), &generation_hash);

    if let Err(err) = &sig_mosaic_supply {
        panic!("{}", err)
    }

    let sig_transaction = &sig_mosaic_supply.unwrap();

    println!("Singer: \t{}", account.public_key_string());
    println!("Hash: \t\t{}", sig_transaction.get_hash());

    let response = client
        .transaction_api()
        .announce(&sig_transaction)
        .await;

    match response {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{}", err),
    }
}
