// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#![deny(warnings)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate failure;

use std::thread::sleep;
use std::time::Duration;

use xpx_chain_api::SiriusClient;
use xpx_chain_sdk::account::{Account, PublicAccount};
use xpx_chain_sdk::message::PlainMessage;
use xpx_chain_sdk::mosaic::Mosaic;
use xpx_chain_sdk::transaction::{
    AggregateTransaction, Deadline, Duration as LockDuration, Hash, LockFundsTransaction,
    SignedTransaction, Transaction, TransferTransaction,
};
use xpx_chain_sdk::Result;

const CO_PRIVATE_KEY: &str = "EE5D1277A862A449173C55454740BEE1A29AB837A97507021340B6EA68909097";
const MS_PUBLIC_KEY: &str = "94E15FAC453553A716D86DADE6EA0EB052B42D62326FB684EA54031540DD674F";

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

    // Deadline default 1 hour
    let deadline = Deadline::default();
    //let deadline = Deadline::new(1, 30, 0);

    let account = Account::from_private_key(CO_PRIVATE_KEY, network_type).unwrap();

    let public_account = PublicAccount::from_public_key(MS_PUBLIC_KEY, network_type).unwrap();

    let ttx_one = TransferTransaction::new(
        deadline,
        account.address(),
        vec![Mosaic::xpx(1)],
        PlainMessage::new("Transfer One From ProximaX Rust SDK"),
        network_type,
    );

    let mut transfer_one = match ttx_one {
        Ok(t) => t,
        Err(err) => panic!("{}", err),
    };
    transfer_one.to_aggregate(public_account.to_owned());

    let ttx_two = TransferTransaction::new(
        deadline,
        public_account.address,
        vec![Mosaic::xpx(15)],
        PlainMessage::new("Transfer Two From ProximaX Rust SDK"),
        network_type,
    );
    let mut transfer_two = match ttx_two {
        Ok(t) => t,
        Err(err) => panic!("{}", err),
    };
    transfer_two.to_aggregate(account.public_account_to_owned());

    let aggregate_bonded =
        AggregateTransaction::new_bonded(deadline, vec![Box::new(transfer_one)], network_type);

    let sig_transaction = account.sign(aggregate_bonded.unwrap(), &generation_hash);

    let sig_tx = match &sig_transaction {
        Ok(sig) => sig,
        Err(err) => panic!("{}", err),
    };

    let lock_fund = lock_fund(&client, &account, sig_tx.get_hash(), generation_hash).await;
    if let Err(err) = &lock_fund {
        panic!("{}", err)
    }

    println!("Singer: \t{}", account.public_key_string());
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
    println!("Hash Lock: \t\t{}", &sig_tx.get_hash());

    let response = client.transaction_api().announce(&sig_tx).await;

    match response {
        Ok(resp) => println!("{}\n", resp),
        Err(err) => panic!("{}\n", err),
    }
    sleep(Duration::from_secs(3));

    for i in 0..6 {
        let response = client
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
