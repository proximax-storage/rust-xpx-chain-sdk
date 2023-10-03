/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::time::{Duration, Instant};

use xpx_chain_sdk::account::Account;
use xpx_chain_sdk::api::ApiNode;
use xpx_chain_sdk::transaction::{
    AggregateTransaction, Deadline, MetadataV2AccountTransaction, Transaction,
};

#[tokio::main]
async fn main() -> Result<(), xpx_chain_sdk::api::error::Error> {
    let start = Instant::now();

    let client = ApiNode::builder()
        .uri("http://bctestnet3.brimstone.xpxsirius.io:3000")
        // .timeout(Duration::from_secs(1))
        .connect_timeout(Duration::from_secs(5))
        .build()?
        .connect()
        .await?;

    let block_info = client.chain_api().get_block_by_height(1).await?;

    // // let network_type = xpx_chain_sdk::network::PUBLIC_TEST;
    let network_type = block_info.network_type;
    let generation_hash = block_info.generation_hash;

    let private_key: &str = "970F8FE51765D1E426C0FE895B7B217FB47C39D049C68EEDFD71FB523812DF10";

    // Create an [Account] from a given Private key.
    let account = Account::from_hex_private_key(private_key, network_type)?;

    println!("{}", account.address_str());

    let mut metadata_txn = MetadataV2AccountTransaction::builder(
        Deadline::default(),
        account.public_account,
        "branch_id",
        format!("{:X}", generation_hash),
        None,
        network_type,
        None,
    )?;
    metadata_txn.set_aggregate(account.public_account);

    let transaction = AggregateTransaction::builder_complete(network_type)
        .inner_transactions(vec![Box::new(metadata_txn)])
        .build()?;

    let sig_transaction = account.sign_transaction(transaction, generation_hash)?;

    let response = client.transaction_api().announce(&sig_transaction).await?;
    println!("{response}");
    println!("Singer: \t{}", account.public_key_to_hex());
    println!("Hash: \t{:X}", sig_transaction.hash);

    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
    Ok(())
}
