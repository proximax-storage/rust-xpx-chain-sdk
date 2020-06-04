// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_apis::SiriusClient;
use xpx_chain_sdk::account::{Account, Address};
use xpx_chain_sdk::blockchain::BlockInfo;
use xpx_chain_sdk::message::PlainMessage;
// use xpx_chain_sdk::mosaic::Mosaic;
use xpx_chain_sdk::transaction::{Deadline, Transaction, TransactionInfo, TransactionStatus, TransferTransaction};
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
        Err(err) => panic!("{}", err)
    };

    println!("Websocket UID: {}", ws_conn.uid());

    if let Err(err) = ws_conn.add_block_handlers(block_handler).await {
        panic!("{}", err)
    }

    if let Err(err) = ws_conn.add_unconfirmed_added_handlers(&account.address(), unconfirmed_added).await {
        panic!("{}", err)
    }

    if let Err(err) = ws_conn.add_confirmed_added_handlers(&account.address(), confirmed_added).await {
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
        vec![],
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


    if let Err(err) = ws_conn.add_status_handlers(&account.address(), status_handler(hash.clone())).await {
        panic!("{}", err)
    }

    if let Err(err) = ws_conn.add_unconfirmed_removed_handlers(&account.address(), unconfirmed_remove(hash.clone())).await {
        panic!("{}", err)
    }

    let listen = tokio::spawn(async move {
        ws_conn.listen().await;
    });

    let response = client
        .transaction_api()
        .announce(&sig_tx)
        .await;

    match response {
        Ok(resp) => println!("{} \n", resp),
        Err(err) => eprintln!("{}", err),
    };

    match listen.await {
        Ok(()) => println!("Sussed"),
        Err(err) => eprintln!("{}", err),
    };
}

fn block_handler(info: BlockInfo) {
    println!("Height: {}, Timestamp: {}", info.height, info.timestamp.to_time());
}

fn status_handler(hash: String) -> Box<dyn Fn(TransactionStatus) + Send + 'static>
{
    Box::new(move |info: TransactionStatus| {
        println!("Content: {}", info.hash);
        if info.hash.eq(&hash) {
            panic!("Status: {}", info.status);
        }
    })
}

fn unconfirmed_added(info: Box<dyn Transaction>) {
    println!("UnconfirmedAdd: {}", info.transaction_hash());
}

fn confirmed_added(info: Box<dyn Transaction>) {
    println!("ConfirmedAdd: {}", info.transaction_hash());
}

fn unconfirmed_remove(hash: String) -> Box<dyn Fn(TransactionInfo) + Send + 'static>
{
    Box::new(move |info: TransactionInfo| {
        let hash_info = info.transaction_hash.unwrap();
        println!("UnconfirmedRemove: {}", hash_info);
        if hash_info.eq(&hash) {
            panic!("Wrong address");
        }
    })
}
