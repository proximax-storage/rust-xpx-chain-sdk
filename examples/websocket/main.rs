// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use std::thread::sleep;
use std::time::Duration;

use failure::_core::fmt::Debug;

use xpx_chain_sdk::account::Address;
use xpx_chain_sdk::blockchain::BlockInfo;
use xpx_chain_sdk::transaction::{Transaction, TransactionInfo, TransactionStatus};
use xpx_chain_websocket::SiriusWebsocketClient;

#[tokio::main]
async fn main() {
    let url = "http://bctestnet1.brimstone.xpxsirius.io:3000";
    //
    let client = SiriusWebsocketClient::new(url).await;
    let mut conn = match client {
        Ok(c) => c,
        Err(e) => panic!("{}", e)
    };

    println!("UID: {}", conn.uid());

    let address = Address::from_raw("VA5KY2EZPFYP3JXFRXC2GWR5LOWL7UISSJ7XCQEJ").unwrap();

    // if let Err(err) = conn.add_block_handlers(block_handler).await {
    //     panic!("{}", err)
    // }

    if let Err(err) = conn.add_status_handlers(&address, status_handler).await {
        panic!("{}", err)
    }

    if let Err(err) = conn.add_unconfirmed_added_handlers(&address, unconfirmed_added).await {
        panic!("{}", err)
    }

    if let Err(err) = conn.add_unconfirmed_removed_handlers(&address, unconfirmed_remove).await {
        panic!("{}", err)
    }

    if let Err(err) = conn.add_confirmed_added_handlers(&address, confirmed_added).await {
        panic!("{}", err)
    }

    tokio::spawn(async move {
        conn.listen().await;
    });

    // pin_mut!(listen);

    // listen.await;

    // select! {
    //     Box = algo => println!("task one completed first"),
    // }
    sleep(Duration::new(120, 0));
}

fn block_handler(info: BlockInfo) {
    println!("Height: {}, Timestamp: {}", info.height, info.timestamp.to_time());
}

fn status_handler(info: TransactionStatus) {
    println!("Content: {}", info.hash);
    panic!("Status: {}", info.status);
}

fn unconfirmed_added(info: Box<dyn Transaction>) {
    println!("UnconfirmedAdd: {}", info.transaction_hash());
}

fn confirmed_added(info: Box<dyn Transaction>) {
    println!("Confirmed: {}", info.transaction_hash());
}

fn unconfirmed_remove(info: TransactionInfo) -> bool {
    println!("UnconfirmedRemove: {}", info.transaction_hash.unwrap());
    true
}
