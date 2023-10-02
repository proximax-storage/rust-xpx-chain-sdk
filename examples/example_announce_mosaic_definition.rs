/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::time::{Duration, Instant};

use xpx_chain_sdk::account::Account;
use xpx_chain_sdk::api::ApiNode;
use xpx_chain_sdk::mosaic::{MosaicNonce, MosaicProperties};
use xpx_chain_sdk::transaction::MosaicDefinitionTransaction;

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

    let chain = client.chain_api().get_block_by_height(1).await?;

    // // let network_type = xpx_chain_sdk::network::PUBLIC_TEST;
    let network_type = chain.network_type;
    let generation_hash = chain.generation_hash;

    let private_key: &str = "7C57C8A6D8767B48F81E8E5D7ECBC76F60D55455A79B4008A20146185F7C8882";

    let account = Account::from_hex_private_key(private_key, network_type).unwrap();

    let nonce = MosaicNonce::random();

    let owner_public_account = account.public_account;

    let properties = MosaicProperties::create(true, true, 2, None);

    let transaction = MosaicDefinitionTransaction::builder(network_type)
        .properties(properties)
        .mosaic_nonce(nonce)
        .owner_public_account(owner_public_account)
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
