// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_api::SiriusClient;
use xpx_chain_sdk::account::{Account, Address};
use xpx_chain_sdk::blockchain::BlockInfo;
use xpx_chain_sdk::message::PlainMessage;
use xpx_chain_sdk::mosaic::Mosaic;
use xpx_chain_sdk::transaction::{
    Deadline, Transaction, TransactionInfo, TransactionStatus, TransferTransaction,
};
use xpx_chain_websocket::SiriusWebsocketClient;

const PRIVATE_KEY: &str = "86258172F90639811F2ABD055747D1E11B55A64B68AED2CEA9A34FBD6C0BE790";

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

    let account = Account::from_private_key(PRIVATE_KEY, network_type).unwrap();

    let ws = SiriusWebsocketClient::new(client.node()).await;
    let mut ws_conn = match ws {
        Ok(ws_client) => ws_client,
        Err(err) => panic!("{}", err),
    };

    println!("Websocket UID: {}", ws_conn.uid());

    if let Err(err) = ws_conn.add_block_handlers(block_handler).await {
        panic!("{}", err)
    }

    if let Err(err) = ws_conn
        .add_unconfirmed_added_handlers(&account.address(), unconfirmed_added)
        .await
    {
        panic!("{}", err)
    }

    if let Err(err) = ws_conn
        .add_confirmed_added_handlers(&account.address(), confirmed_added)
        .await
    {
        panic!("{}", err)
    }

    // Deadline default 1 hour
    // let deadline = Deadline::new(1, 0, 0);
    let deadline = Deadline::default();

    let recipient = Address::from_raw("VC4A3Z6ALFGJPYAGDK2CNE2JAXOMQKILYBVNLQFS").unwrap();

    let message = PlainMessage::new("Transfer From ProximaX Rust SDK");

    let transfer_transaction = TransferTransaction::new(
        deadline,
        recipient,
        vec![Mosaic::xpx_relative(10)],
        message,
        network_type,
    );

    if let Err(err) = &transfer_transaction {
        panic!("{}", err)
    }

    let sig_transaction = account.sign(transfer_transaction.unwrap(), &generation_hash);

    let sig_tx = match &sig_transaction {
        Ok(sig) => sig,
        Err(err) => panic!("{}", err),
    };

    let hash = sig_tx.get_hash();

    println!("Singer: \t{}", account.public_key_string());
    println!("Hash: \t\t{}", hash);

    if let Err(err) = ws_conn
        .add_status_handlers(&account.address(), status_handler(hash.clone()))
        .await
    {
        panic!("{}", err)
    }

    if let Err(err) = ws_conn
        .add_unconfirmed_removed_handlers(&account.address(), unconfirmed_remove(hash.clone()))
        .await
    {
        panic!("{}", err)
    }

    let listen = tokio::spawn(async move {
        if let Err(err) = ws_conn.listen().await {
            eprintln!("{}", err)
        }
    });

    let response = client.transaction_api().announce(&sig_tx).await;

    match response {
        Ok(resp) => println!("{} \n", resp),
        Err(err) => eprintln!("{}", err),
    };

    match listen.await {
        Ok(()) => println!("finished"),
        Err(err) => eprintln!("{}", err),
    };
}

fn block_handler(info: BlockInfo) -> bool {
    println!(
        "Height: {}, Timestamp: {}",
        info.height,
        info.timestamp.to_time()
    );
    false
}

fn status_handler(hash: String) -> Box<dyn Fn(TransactionStatus) -> bool + Send + 'static> {
    Box::new(move |info: TransactionStatus| {
        println!("Content: {}", info.hash);
        if info.hash.eq(&hash) {
            println!("Status: {}", info.status);
            return true;
        };
        false
    })
}

fn unconfirmed_added(info: Box<dyn Transaction>) -> bool {
    println!("UnconfirmedAdd: {}", info.transaction_hash());
    false
}

fn confirmed_added(info: Box<dyn Transaction>) -> bool {
    println!("\n");
    println!("Confirmed Hash: {}", info.transaction_hash());
    println!("Confirmed Entity_type: {}", info.entity_type().to_string());
    println!("Confirmed Height: {}", info.height());
    println!("\n");

    false
}

fn unconfirmed_remove(hash: String) -> Box<dyn Fn(TransactionInfo) -> bool + Send + 'static> {
    Box::new(move |info: TransactionInfo| {
        let hash_info = info.hash.unwrap();
        println!("UnconfirmedRemove: {}", hash_info);
        if hash_info.eq(&hash) {
            true;
        };
        false
    })
}
