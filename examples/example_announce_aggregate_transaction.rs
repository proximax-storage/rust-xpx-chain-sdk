/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::time::Instant;

use xpx_chain_sdk::account::Account;
use xpx_chain_sdk::api::ApiNode;
use xpx_chain_sdk::message::PlainMessage;
use xpx_chain_sdk::transaction::{
    AggregateTransaction, Deadline, Transaction, TransferTransaction,
};

#[tokio::main]
async fn main() -> Result<(), xpx_chain_sdk::api::error::Error> {
    let start = Instant::now();

    let client = ApiNode::from_static("http://api-2.testnet2.xpxsirius.io:3000")
        .connect()
        .await?;

    let chain = client.chain_api().get_block_by_height(1).await?;

    // // let network_type = xpx_chain_sdk::network::PUBLIC_TEST;
    let network_type = chain.network_type;
    let generation_hash = chain.generation_hash;

    let private_key: &str = "26b64cb10f005e5988a36744ca19e20d835ccc7c105aaa5f3b212da593180930";

    let account = Account::from_hex_private_key(private_key, network_type).unwrap();
    let recipient_a = Account::random(network_type).to_address();
    let recipient_b = Account::random(network_type).to_address();

    let mut transfer_a = TransferTransaction::create(
        Deadline::default(),
        recipient_a,
        vec![],
        PlainMessage::create("Transfer A From ProximaX Rust SDK"),
        network_type,
        None,
    )?;
    transfer_a.set_aggregate(account.public_account);

    let mut transfer_b = TransferTransaction::create(
        Deadline::default(),
        recipient_b,
        vec![],
        PlainMessage::create("Transfer B From ProximaX Rust SDK"),
        network_type,
        None,
    )?;
    transfer_b.set_aggregate(account.public_account);

    let aggregate_complete = AggregateTransaction::create_complete(
        Deadline::default(),
        vec![Box::new(transfer_a), Box::new(transfer_b)],
        network_type,
        None,
    )?;

    let sig_transaction = account.sign_transaction(aggregate_complete, generation_hash)?;

    let response = client.transaction_api().announce(&sig_transaction).await?;
    println!("{response}");
    println!("Singer: \t{}", account.public_key_to_hex());
    println!("Hash: \t{:X}", sig_transaction.hash);

    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
    Ok(())
}
