#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_apis::SiriusClient;
use xpx_chain_sdk::account::Account;
use xpx_chain_sdk::multisig::{CosignatoryModification, MultisigModificationType};
use xpx_chain_sdk::transaction::{
    AggregateTransaction, Deadline, ModifyMultisigAccountTransaction, Transaction,
};

/// Future multiSig private key
const MULTI_SIG_PRIVATE_KEY: &str =
    "3B5550B5CB19C893694FC49B461CE489BF9588BE16DBE8DC29CF06338133DEE7";

/// Cosignature public keys
const COSIGNATORY_ONE_PRIVATE_KEY: &str =
    "16DBE8DC29CF06338133DEE64FC49B461CE489BF9588BE3B9670B5CB19C89369";
const COSIGNATORY_TWO_PRIVATE_KEY: &str =
    "461CE489BF9588BE3B9670B5CB19C8936916DBE8DC29CF06338133DEE64FC49B";
const COSIGNATORY_THREE_PRIVATE_KEY: &str =
    "29CF06338133DEE64FC49BCB19C8936916DBE8DC461CE489BF9588BE3B9670B5";

/// Minimal approval count
const MINIMAL_APPROVAL: i8 = 2;

/// Minimal removal count
const MINIMAL_REMOVAL: i8 = 3;

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

    let multi_sig_account = Account::from_private_key(MULTI_SIG_PRIVATE_KEY, network_type).unwrap();

    let cosignatory_one =
        Account::from_private_key(COSIGNATORY_ONE_PRIVATE_KEY, network_type).unwrap();
    let cosignatory_two =
        Account::from_private_key(COSIGNATORY_TWO_PRIVATE_KEY, network_type).unwrap();
    let cosignatory_three =
        Account::from_private_key(COSIGNATORY_THREE_PRIVATE_KEY, network_type).unwrap();

    let modify_multi_sig_account_new = ModifyMultisigAccountTransaction::new(
        deadline,
        MINIMAL_APPROVAL,
        MINIMAL_REMOVAL,
        vec![
            CosignatoryModification::new(
                MultisigModificationType::Add,
                cosignatory_one.public_account_to_owned(),
            ),
            CosignatoryModification::new(
                MultisigModificationType::Add,
                cosignatory_two.public_account_to_owned(),
            ),
            CosignatoryModification::new(
                MultisigModificationType::Add,
                cosignatory_three.public_account_to_owned(),
            ),
        ],
        network_type,
    );
    let mut modify_multi_sig_account = match modify_multi_sig_account_new {
        Ok(t) => t,
        Err(err) => panic!("{}", err),
    };

    modify_multi_sig_account.to_aggregate(multi_sig_account.public_account_to_owned());

    let aggregate_bonded = AggregateTransaction::new_complete(
        deadline,
        vec![Box::new(modify_multi_sig_account)],
        network_type,
    );
    if let Err(err) = aggregate_bonded {
        panic!("{}", err)
    }

    let sig_transaction = multi_sig_account.sign_with_cosignatories(
        aggregate_bonded.unwrap(),
        vec![cosignatory_one, cosignatory_two, cosignatory_three],
        &generation_hash,
    );

    let sig_tx = match &sig_transaction {
        Ok(sig) => sig,
        Err(err) => panic!("{}", err),
    };

    println!("Singer: \t{}", multi_sig_account.public_key_string());
    println!("Hash: \t\t{}", sig_tx.get_hash());

    let response = client
        .transaction_api()
        .announce(&sig_tx)
        .await;

    match response {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{}", err),
    }
}
