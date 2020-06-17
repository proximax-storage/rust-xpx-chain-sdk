// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use xpx_chain_sdk::account::Address;
use xpx_chain_sdk::blockchain::BlockInfo;
use xpx_chain_sdk::multisig::CosignatureInfo;
use xpx_chain_sdk::transaction::{
    AbsTransaction, AggregateTransaction, Transaction, TransactionInfo, TransactionStatus,
};
use xpx_chain_websocket::SiriusWebsocketClient;

#[tokio::main]
async fn main() {
    let url = "http://bctestnet1.brimstone.xpxsirius.io:3000";
    
    let client = SiriusWebsocketClient::new(url).await;
    let mut ws = match client {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };
    
    println!("WebSocket UID: {}", ws.uid());
    
    let address = Address::from_raw("VB4JKB-IUALJY-LYNRB2-QJM3TP-DGZDOG-HSVKDI-BR4R").unwrap();
    
    if let Err(err) = ws.add_block_handlers(block_handler).await {
        panic!("{}", err)
    }
    
    if let Err(err) = ws.add_status_handlers(&address, status_handler).await {
        panic!("{}", err)
    }
    
    if let Err(err) = ws
        .add_unconfirmed_added_handlers(&address, unconfirmed_added)
        .await
    {
        panic!("{}", err)
    }
    
    if let Err(err) = ws
        .add_unconfirmed_removed_handlers(&address, unconfirmed_remove)
        .await
    {
        panic!("{}", err)
    }
    
    if let Err(err) = ws
        .add_confirmed_added_handlers(&address, confirmed_added)
        .await
    {
        panic!("{}", err)
    }
    
    if let Err(err) = ws.add_partial_added_handlers(&address, partial_added).await {
        panic!("{}", err)
    }
    
    if let Err(err) = ws
        .add_partial_removed_handlers(&address, partial_remove)
        .await
    {
        panic!("{}", err)
    }
    
    if let Err(err) = ws.add_cosignature_handlers(&address, cosignature).await {
        panic!("{}", err)
    }
    
    let listen = tokio::spawn(async move {
        if let Err(err) = ws.listen().await {
            eprintln!("{}", err)
        }
    });
    
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

fn status_handler(info: TransactionStatus) -> bool {
    println!("Content Hash: {}", info.hash);
    println!("Status: {}", info.status);
    true
}

fn unconfirmed_added(info: Box<dyn Transaction>) -> bool {
    println!("UnconfirmedAdd Hash: {}", info.transaction_hash());
    println!("UnconfirmedAdd Type: {}", info.entity_type());
    
    false
}

fn confirmed_added(info: Box<dyn Transaction>) -> bool {
    println!("Confirmed Hash: {}", info.transaction_hash());
    println!("UnconfirmedAdd Type: {}", info.entity_type());
    
    false
}

fn unconfirmed_remove(info: TransactionInfo) -> bool {
    println!("UnconfirmedRemove Hash: {}", info.transaction_hash());
    false
}

fn partial_added(info: AggregateTransaction) -> bool {
    println!("PartialAdded Hash: {}", info.transaction_hash());
    false
}

fn partial_remove(info: TransactionInfo) -> bool {
    println!("PartialRemove Hash: {}", info.transaction_hash());
    false
}

fn cosignature(info: CosignatureInfo) -> bool {
    println!("Cosignature Hash: {}", info.transaction_hash());
    false
}

fn test_handler(hash: String) -> Box<dyn Fn(Box<dyn Transaction>) -> bool + Send + 'static> {
    Box::new(move |info: Box<dyn Transaction + 'static>| {
        let hash_info = info.transaction_hash();
        
        println!("Test Hash: {}", hash);
        println!("Unconfirmed Hash: {}", hash_info);
        
        if hash_info.eq(&hash) {
            true;
        };
        
        false
    })
}
