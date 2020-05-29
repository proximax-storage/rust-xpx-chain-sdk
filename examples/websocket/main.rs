// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use std::thread::sleep;

use failure::_core::time::Duration;
use tokio::task;

use xpx_chain_sdk::account::Address;
use xpx_chain_sdk::blockchain::BlockInfo;
use xpx_chain_sdk::transaction::TransactionStatus;
use xpx_chain_websocket::SiriusWebsocketClient;

#[tokio::main]
async fn main() {
    let url = "ws://bctestnet1.brimstone.xpxsirius.io:3000/ws";

    let client = SiriusWebsocketClient::new(url).await;
    let mut conn = match client {
        Ok(c) => c,
        Err(e) => panic!(e)
    };

    println!("{:?}", conn.uid().uid);

    let address = Address::from_raw("VCDWDMUZOHZMHJO6LA772WMPP5IVJPU226UKAJUL").unwrap();

    conn.add_block_handlers(block_handler).await;

    conn.add_status_handlers(address, status_handler).await;

    task::spawn(async move {
        conn.listen().await;
    }).await;
}

fn block_handler(info: Address) {
    println!("{}", info.address)
}

fn status_handler(info: TransactionStatus) {
    println!("{}", info)
}