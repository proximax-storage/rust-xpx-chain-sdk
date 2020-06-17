// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#![deny(warnings)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate failure;

use std::thread::sleep;
use std::time::Duration;
use xpx_chain_apis::SiriusClient;
use xpx_chain_sdk::account::{Account, PublicAccount};
use xpx_chain_sdk::mosaic::Mosaic;
use xpx_chain_sdk::multisig::{CosignatoryModification, MultisigModificationType};
use xpx_chain_sdk::Result;
use xpx_chain_sdk::transaction::{
    AggregateTransaction, Deadline, Duration as LockDuration, Hash, LockFundsTransaction,
    ModifyMultisigAccountTransaction, SignedTransaction, Transaction,
};

/// Future multiSig private key
const MULTI_SIG_PRIVATE_KEY: &str =
    "3B7660B6CB19C893694FC49B461CE489BF9588BE16DBE6DC29CF06338133DEE8";

/// Cosignature public keys
const COSIGNATORY_ONE_PUBLIC_KEY: &str =
    "4543e65720c9ed6d9ac5fb360aebd24223f5e08d442416b70bec0b4a4446d5a4";
const COSIGNATORY_TWO_PUBLIC_KEY: &str =
    "2ef33fc8b00483c3aa4ad3344ed368a0053c273b5155b83c80efbb12925938ec";
const COSIGNATORY_THREE_PUBLIC_KEY: &str =
    "E1BADE372F48CAB5F00C40F55408DD5068BBE4B9EF2E1B680818A942C69CD0C8";

/// Minimal approval count
const MINIMAL_APPROVAL: i8 = 2;

/// Minimal removal count
const MINIMAL_REMOVAL: i8 = 1;

#[tokio::main]
async fn main() {
    let node_url = vec!["http://bctestnet3.brimstone.xpxsirius.io:3000"];

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
    
    let multi_sig_account = Account::from_private_key(MULTI_SIG_PRIVATE_KEY, network_type).unwrap();
    
    println!("MSA: {}", multi_sig_account.address_string());
    
    let cosignatory_one =
        PublicAccount::from_public_key(COSIGNATORY_ONE_PUBLIC_KEY, network_type).unwrap();
    let cosignatory_two =
        PublicAccount::from_public_key(COSIGNATORY_TWO_PUBLIC_KEY, network_type).unwrap();
    let cosignatory_three =
        PublicAccount::from_public_key(COSIGNATORY_THREE_PUBLIC_KEY, network_type).unwrap();
    
    let modify_multi_sig_account_new = ModifyMultisigAccountTransaction::new(
        deadline,
        MINIMAL_APPROVAL,
        MINIMAL_REMOVAL,
        vec![
            CosignatoryModification::new(MultisigModificationType::Add, cosignatory_one),
            CosignatoryModification::new(MultisigModificationType::Add, cosignatory_two),
            CosignatoryModification::new(MultisigModificationType::Add, cosignatory_three),
        ],
        network_type,
    );
    let mut modify_multi_sig_account = match modify_multi_sig_account_new {
        Ok(t) => t,
        Err(err) => panic!("{}", err),
    };

    modify_multi_sig_account.to_aggregate(multi_sig_account.public_account_to_owned());

    let aggregate_bonded = AggregateTransaction::new_bonded(
        deadline,
        vec![Box::new(modify_multi_sig_account)],
        network_type,
    );
    if let Err(err) = aggregate_bonded {
        panic!("{}", err)
    }

    let sig_transaction = multi_sig_account.sign(aggregate_bonded.unwrap(), &generation_hash);

    let sig_tx = match &sig_transaction {
        Ok(sig) => sig,
        Err(err) => panic!("{}", err),
    };
    
    let lock_fund = lock_fund(
        &client,
        &multi_sig_account,
        sig_tx.get_hash(),
        generation_hash,
    )
        .await;
    if let Err(err) = &lock_fund {
        panic!("{}", err)
    }

    println!("Singer: \t{}", multi_sig_account.public_key_string());
    println!("Hash: \t\t{}", sig_tx.get_hash());

    let response = client
        .transaction_api()
        .announce_aggregate_bonded(&sig_tx)
        .await;

    match response {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{}", err),
    }
}

async fn lock_fund(
    client: &Box<SiriusClient>,
    account: &Account,
    signed_hash: Hash,
    generation_hash: String,
) -> Result<()> {
    // let network_type = xpx_chain_sdk::network::PUBLIC_TEST;
    let network_type = client.network_type();
    
    // Deadline default 1 hour
    // let deadline = Deadline::new(1, 0, 0);
    let deadline = Deadline::default();
    
    let lock_transaction = LockFundsTransaction::new(
        deadline,
        Mosaic::xpx_relative(10),
        LockDuration::new(100),
        SignedTransaction::from_hash(signed_hash),
        network_type,
    )?;

    let sig_tx = account.sign(lock_transaction, &generation_hash)?;

    println!("Singer Lock: \t{}", account.public_key_string());
    println!("Hash Lock: \t\t{}", sig_tx.get_hash());

    let response = client.to_owned().transaction_api().announce(&sig_tx).await;
    match response {
        Ok(resp) => println!("{}\n", resp),
        Err(err) => panic!("{}\n", err),
    }
    sleep(Duration::from_secs(3));

    for i in 0..6 {
        let response = client
            .to_owned()
            .transaction_api()
            .get_transaction_status(&sig_tx.get_hash())
            .await;

        if let Ok(status) = response {
            if !status.is_success() {
                bail!("{}", status.status)
            } else if status.is_confirmed() {
                println!("Status: {}\n", status.group);
                break;
            }
            println!("Status: {}", status.group)
        }

        ensure!(i != 6, "time out.");

        sleep(Duration::from_secs(10));
    }
    Ok(())
}
