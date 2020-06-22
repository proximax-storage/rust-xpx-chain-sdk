// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_api::SiriusClient;
use xpx_chain_sdk::account::Account;
use xpx_chain_sdk::namespace::NamespaceId;
use xpx_chain_sdk::transaction::{Deadline, RegisterNamespaceTransaction};
use xpx_chain_sdk::Uint64;

const PRIVATE_KEY: &str = "EE5D1277A862A449173C55454740BEE1A29AB837A97507021340B6EA68909097";

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

    let register_namespace_root =
        RegisterNamespaceTransaction::create_root(deadline, "rust", Uint64::new(100), network_type);

    let namespace_root = match register_namespace_root {
        Ok(register_namespace) => register_namespace,
        Err(err) => panic!("{}", err),
    };

    let sig_transaction_root = account.sign(namespace_root, &generation_hash);

    if let Err(err) = &sig_transaction_root {
        panic!("{}", err)
    }

    let sig_transaction = &sig_transaction_root.unwrap();

    println!("Singer: \t{}", account.public_key_string());
    println!("Hash: \t\t{}", sig_transaction.get_hash());

    let response_root = client.transaction_api().announce(&sig_transaction).await;

    match response_root {
        Ok(response) => println!("{}\n", response),
        Err(err) => eprintln!("{}", err),
    }

    let register_namespace_sub = RegisterNamespaceTransaction::create_sub(
        deadline,
        "latam",
        NamespaceId::from_name("rust").unwrap(),
        network_type,
    );

    if let Err(err) = &register_namespace_sub {
        panic!("{}", err)
    }

    let sig_transaction_sub = account.sign(register_namespace_sub.unwrap(), &generation_hash);

    if let Err(err) = &sig_transaction_sub {
        panic!("{}", err)
    }

    let sig_transaction = &sig_transaction_sub.unwrap();

    println!("Singer: \t{}", account.public_key_string());
    println!("Hash: \t\t{}", sig_transaction.get_hash());

    let response_sub = client.transaction_api().announce(&sig_transaction).await;

    match response_sub {
        Ok(response) => println!("{}", response),
        Err(err) => eprintln!("{}", err),
    }
}
