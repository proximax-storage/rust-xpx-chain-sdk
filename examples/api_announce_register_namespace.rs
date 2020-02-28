#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::Client;

use xpx_chain_sdk::apis::sirius_client::SiriusClient;
use xpx_chain_sdk::models::account::Account;
use xpx_chain_sdk::models::network::PUBLIC_TEST;
use xpx_chain_sdk::models::transaction::{Deadline, RegisterNamespaceTransaction};
use xpx_chain_sdk::models::Uint64;

#[tokio::main]
async fn main() {
    let node = "http://bctestnet3.brimstone.xpxsirius.io:3000";

    let client = SiriusClient::new(node, Client::new());

    let network_type = PUBLIC_TEST;

    // Deadline default 1 hour
    let deadline = Deadline::default();

    let private_key = "5D3E959EB0CD69CC1DB6E9C62CB81EC52747AB56FA740CF18AACB5003429AD2E";

    let account = Account::from_private_key(private_key, network_type).unwrap();

    let register_namespace_root = RegisterNamespaceTransaction::create_root(
        deadline,
        "rustnamespace",
        Uint64::new(1),
        network_type
    );

    let register_namespace_root_tx = loop {
        match &register_namespace_root {
            Ok(definition) => break definition,
            Err(err) => panic!("{}", err),
        }
    };

    let sig_transaction_root = account.sign(
        register_namespace_root_tx,
        "56D112C98F7A7E34D1AEDC4BD01BC06CA2276DD546A93E36690B785E82439CA9".to_owned());

    let sig_root_tx = loop {
        match &sig_transaction_root {
            Ok(sig) => break sig,
            Err(err) => panic!("{}", err),
        }
    };

    println!("Singer: \t{}", account.public_account.public_key.to_uppercase());
    println!("Hash: \t\t{}", sig_root_tx.hash.to_uppercase());

    let response_root = client.clone().transaction.announce_transaction(&sig_root_tx).await;

    match response_root {
        Ok(resp) => println!("{}\n", resp),
        Err(err) => panic!("{:?}", err),
    }


    let register_namespace_sub = RegisterNamespaceTransaction::create_sub(
        deadline,
        "latam",
        register_namespace_root_tx.namespace_id,
        network_type
    );

    let register_namespace_sub_tx = loop {
        match &register_namespace_sub {
            Ok(definition) => break definition,
            Err(err) => panic!("{}", err),
        }
    };

    let sig_transaction_sub = account.sign(
        register_namespace_sub_tx,
        "56D112C98F7A7E34D1AEDC4BD01BC06CA2276DD546A93E36690B785E82439CA9".to_owned());

    let sig_sub_tx = loop {
        match &sig_transaction_sub {
            Ok(sig) => break sig,
            Err(err) => panic!("{}", err),
        }
    };

    println!("Singer: \t{}", account.public_account.public_key.to_uppercase());
    println!("Hash: \t\t{}", sig_sub_tx.hash.to_uppercase());

    let response_sub = client.transaction.announce_transaction(&sig_sub_tx).await;

    match response_sub {
        Ok(resp) => println!("{}", resp),
        Err(err) => panic!("{:?}", err),
    }
}
