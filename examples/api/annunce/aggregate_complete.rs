// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_api::SiriusClient;
use xpx_chain_sdk::account::{Account, Address};
use xpx_chain_sdk::message::PlainMessage;
use xpx_chain_sdk::mosaic::Mosaic;
use xpx_chain_sdk::transaction::{
    AggregateTransaction, Deadline, Transaction, TransferTransaction,
};

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
    let deadline = Deadline::default();
    //let deadline = Deadline::new(1, 30, 0);

    let account = Account::from_private_key(PRIVATE_KEY, network_type).unwrap();

    let transfer_transaction_a = TransferTransaction::new(
        deadline,
        Address::from_raw("VB4JKB-IUALJY-LYNRB2-QJM3TP-DGZDOG-HSVKDI-BR4R").unwrap(),
        vec![Mosaic::xpx_relative(15)],
        PlainMessage::new("Transfer A From ProximaX Rust SDK"),
        network_type,
    );
    let mut transfer_a = match transfer_transaction_a {
        Ok(t) => t,
        Err(err) => panic!("{}", err),
    };
    transfer_a.to_aggregate(account.public_account_to_owned());

    let transfer_transaction_b = TransferTransaction::new(
        deadline,
        Address::from_raw("VC4A3Z6ALFGJPYAGDK2CNE2JAXOMQKILYBVNLQFS").unwrap(),
        vec![Mosaic::xpx(15)],
        PlainMessage::new("Transfer B From ProximaX Rust SDK"),
        network_type,
    );

    let mut transfer_b = match transfer_transaction_b {
        Ok(t) => t,
        Err(err) => panic!("{}", err),
    };
    transfer_b.to_aggregate(account.public_account_to_owned());

    let aggregate_complete = AggregateTransaction::new_complete(
        deadline,
        vec![Box::new(transfer_a), Box::new(transfer_b)],
        network_type,
    );

    let sig_transaction = account.sign(aggregate_complete.unwrap(), &generation_hash);

    let sig_tx = match &sig_transaction {
        Ok(sig) => sig,
        Err(err) => panic!("{}", err),
    };

    println!("Singer: \t{}", account.public_key_string());
    println!("Hash: \t\t{}", sig_tx.get_hash());

    let response = client.transaction_api().announce(&sig_tx).await;

    match response {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{}", err),
    }
}
