#![deny(warnings)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate failure;

use std::fmt::Debug;
use std::thread::sleep;
use std::time::Duration;

use hyper::{Client, client::connect::Connect};

use xpx_chain_sdk::account::{Account, PublicAccount};
use xpx_chain_sdk::mosaic::Mosaic;
use xpx_chain_sdk::network::PUBLIC_TEST;
use xpx_chain_sdk::Result;
use xpx_chain_sdk::sirius_client::SiriusClient;
use xpx_chain_sdk::transaction::{AggregateTransaction, Deadline, Duration  as LockDuration, Hash, LockFundsTransaction, SignedTransaction, Transaction, ModifyMultisigAccountTransaction};
use xpx_chain_sdk::multisig::{CosignatoryModification, MultisigModificationType};

const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";

/// Future multiSig private key
const MULTI_SIG_PRIVATE_KEY: &str = "3B7660B5CB19C893694FC49B461CE489BF9588BE16DBE8DC29CF06338133DEE6";

/// Cosignature public keys
const COSIGNATORY_ONE_PRIVATE_KEY: &str = "16DBE8DC29CF06338133DEE64FC49B461CE489BF9588BE3B9670B5CB19C89369";
const COSIGNATORY_TWO_PRIVATE_KEY: &str = "461CE489BF9588BE3B9670B5CB19C8936916DBE8DC29CF06338133DEE64FC49B";
const COSIGNATORY_THREE_PRIVATE_KEY: &str = "29CF06338133DEE64FC49BCB19C8936916DBE8DC461CE489BF9588BE3B9670B5";

/// Minimal approval count
const MINIMAL_APPROVAL: i8 = 2;

/// Minimal removal count
const MINIMAL_REMOVAL: i8 = 3;

#[tokio::main]
async fn main() {
    let sirius_client = SiriusClient::new(NODE_URL, Client::new()).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let generation_hash = client.generation_hash();

    // let network_type = client.network_type().await;
    let network_type = PUBLIC_TEST;

    // Deadline default 1 hour
    let deadline = Deadline::default();
    //let deadline = Deadline::new(1, 30, 0);

    let multi_sig_account = Account::from_private_key(MULTI_SIG_PRIVATE_KEY, network_type).unwrap();

    let cosignatory_one = PublicAccount::from_public_key(COSIGNATORY_ONE_PRIVATE_KEY, network_type).unwrap();
    let cosignatory_two = PublicAccount::from_public_key(COSIGNATORY_TWO_PRIVATE_KEY, network_type).unwrap();
    let cosignatory_three = PublicAccount::from_public_key(COSIGNATORY_THREE_PRIVATE_KEY, network_type).unwrap();

    let modify_multi_sig_account_new = ModifyMultisigAccountTransaction::new(
        deadline,
        MINIMAL_APPROVAL,
        MINIMAL_REMOVAL,
        vec![
            CosignatoryModification::new(MultisigModificationType::Add, cosignatory_one),
            CosignatoryModification::new(MultisigModificationType::Add, cosignatory_two),
            CosignatoryModification::new(MultisigModificationType::Add, cosignatory_three)
        ],
        network_type,
    );
    let mut modify_multi_sig_account = match modify_multi_sig_account_new {
        Ok(t) => t,
        Err(err) => panic!("{}", err)
    };

    modify_multi_sig_account.to_aggregate(multi_sig_account.to_owned().public_account);

    let aggregate_bonded = AggregateTransaction::new_bonded(
        deadline,
        vec![Box::new(modify_multi_sig_account )],
        network_type
    );
    if let Err(err) = aggregate_bonded {
        panic!("{}", err)
    }

    let sig_transaction = multi_sig_account.sign(
        aggregate_bonded.unwrap(), &generation_hash);

    let sig_tx = match &sig_transaction {
        Ok(sig) => sig,
        Err(err) => panic!("{}", err),
    };

    let lock_fund = lock_fund(&client, &multi_sig_account, sig_tx.clone().hash, generation_hash).await;
    if let Err(err) = &lock_fund {
        panic!("{}", err)
    }

    println!("Singer: \t{}", multi_sig_account.public_account.public_key.to_uppercase());
    println!("Hash: \t\t{}", &sig_tx.get_hash().to_uppercase());

    let response = client.transaction.announce_aggregate_bonded(&sig_tx).await;

    match response {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{}", err),
    }
}

async fn lock_fund<C: Connect>(client: &Box<SiriusClient<C>>, account: &Account,
                               signed_hash: Hash, generation_hash: String) -> Result<()>
    where
        C: Clone + Send + Sync + Debug + 'static
{
//    let network_type = client.network_type().await;
    let network_type = PUBLIC_TEST;

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

    println!("Singer Lock: \t{}", account.public_account.public_key.to_uppercase());
    println!("Hash Lock: \t\t{}", &sig_tx.get_hash().to_uppercase());

    let response = client.to_owned().transaction.announce(&sig_tx).await;
    match response {
        Ok(resp) => println!("{}\n", resp),
        Err(err) => panic!("{}\n", err),
    }
    sleep(Duration::from_secs(3));

    for i in 0..6 {
        let response = client.to_owned().transaction.get_transaction_status(&sig_tx.get_hash()).await;
        if let Ok(status) = response {
            if !status.is_success() {
                bail!("{}", status.status)
            } else if status.is_confirmed() {
                println!("Status: {}\n", status.group);
                break
            }
            println!("Status: {}", status.group)
        }

        ensure!( i != 6 , "time out." );

        sleep(Duration::from_secs(10));
    };
    Ok(())
}