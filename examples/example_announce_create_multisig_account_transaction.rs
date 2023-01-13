/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use anyhow::anyhow;
use std::thread::sleep;
use std::time::{Duration, Instant};

use xpx_chain_sdk::account::{Account, PublicAccount};
use xpx_chain_sdk::api::{ApiNode, Client};
use xpx_chain_sdk::mosaic::Mosaic;
use xpx_chain_sdk::multisig::{CosignatoryModification, MultisigModificationType};
use xpx_chain_sdk::network::NetworkType;
use xpx_chain_sdk::transaction::{
    AggregateTransaction, Deadline, LockFundsTransaction, ModifyMultisigAccountTransaction,
    SignedTransaction, Transaction,
};
use xpx_chain_sdk::TransactionHash;

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

    let cosignatory_one_public_key =
        "738b2b466f718b14a1bf40fc6ce993e464eba6894dadedf1661ef6f79eef288a";
    let cosignatory_two_public_key =
        "8d0e5deb5042ad3bf59dd82882eedc8c55cf5c5cf14de4d67a1338e70bd112ed";
    let cosignatory_three_public_key =
        "fc02cce49b057f646b32f4aa1867b2259a09e5724d8a042fcce5f24c371c1c8c";

    let cosignatory_one = PublicAccount::from_public_key(cosignatory_one_public_key, network_type)?;
    let cosignatory_two = PublicAccount::from_public_key(cosignatory_two_public_key, network_type)?;
    let cosignatory_three =
        PublicAccount::from_public_key(cosignatory_three_public_key, network_type)?;

    let min_approval_delta: i8 = 2;
    let min_removal_delta: i8 = 1;

    let modifications = vec![
        CosignatoryModification::create(MultisigModificationType::Add, cosignatory_one),
        CosignatoryModification::create(MultisigModificationType::Add, cosignatory_two),
        CosignatoryModification::create(MultisigModificationType::Add, cosignatory_three),
    ];

    let deadline = Deadline::default();
    let mut multisig_account_transaction = ModifyMultisigAccountTransaction::create(
        deadline,
        min_approval_delta,
        min_removal_delta,
        modifications,
        network_type,
        None,
    )?;
    multisig_account_transaction.set_aggregate(account.public_account);

    let transaction = AggregateTransaction::create_bonded(
        deadline,
        vec![Box::new(multisig_account_transaction)],
        network_type,
        None,
    )?;

    let sig_transaction = account.sign_transaction(transaction, generation_hash)?;

    let _ = lock_fund(
        client.clone(),
        &account,
        sig_transaction.clone(),
        network_type,
        generation_hash,
    )
    .await?;

    let response = client
        .transaction_api()
        .announce_aggregate_bonded(&sig_transaction)
        .await?;
    println!("{response}");
    println!("Singer: \t{}", account.public_key_to_hex());
    println!("Hash: \t{:X}", sig_transaction.get_hash());

    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
    Ok(())
}

async fn lock_fund(
    client: Client,
    account: &Account,
    signed_tx: SignedTransaction,
    network_type: NetworkType,
    generation_hash: TransactionHash,
) -> Result<(), xpx_chain_sdk::api::error::Error> {
    // Deadline default 1 hour
    // let deadline = Deadline::new(1, 0, 0);
    let deadline = Deadline::default();

    let lock_transaction = LockFundsTransaction::create(
        deadline,
        Mosaic::xpx_relative(10),
        1000,
        signed_tx,
        network_type,
        None,
    )?;

    let sig_tx = account.sign_transaction(lock_transaction, generation_hash)?;

    println!("Singer Lock: \t{}", account.public_key_to_hex());
    println!("Hash Lock: \t\t{:X}", sig_tx.get_hash());

    let _ = client.transaction_api().announce(&sig_tx).await?;

    sleep(Duration::from_secs(3));
    // 188939084794
    // 188939084794
    for i in 0..6 {
        let response = client
            .to_owned()
            .transaction_api()
            .get_transaction_status(sig_tx.get_hash())
            .await;

        if let Ok(status) = response {
            println!("LockFundsTransaction Status: {}", status.group);
            if !status.is_success() {
                return Err(xpx_chain_sdk::api::error::Error::Failure(anyhow!(
                    status.status
                )));
            } else if status.is_confirmed() {
                break;
            }
        }

        if i == 6 {
            return Err(xpx_chain_sdk::api::error::Error::Failure(anyhow!(
                "time out."
            )));
        }

        sleep(Duration::from_secs(10));
    }
    Ok(())
}
