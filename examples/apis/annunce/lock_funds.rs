#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_apis::SiriusClient;
use xpx_chain_sdk::account::Account;
use xpx_chain_sdk::mosaic::Mosaic;
use xpx_chain_sdk::transaction::{
    Deadline, Duration, EntityTypeEnum, LockFundsTransaction, SignedTransaction,
};

const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";
const PRIVATE_KEY: &str = "6D3E959EB0CD69CC1DB6E9C62CB81EC52747AB56FA740CF18AACB5003429AD2E";

#[tokio::main]
async fn main() {
    let sirius_client = SiriusClient::new(NODE_URL).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let generation_hash = client.generation_hash();

    // let network_type = xpx_chain_sdk::network::PUBLIC_TEST;
    let network_type = client.network_type();

    // Deadline default 1 hour
    // let deadline = Deadline::new(1, 0, 0);
    let deadline = Deadline::default();

    let account = Account::from_private_key(PRIVATE_KEY, network_type).unwrap();

    let lock_transaction = LockFundsTransaction::new(
        deadline,
        Mosaic::xpx_relative(10),
        Duration::new(100),
        SignedTransaction {
            entity_type: EntityTypeEnum::AggregateBonded,
            payload: None,
            hash: "1731245d3bd7af58e065a6971e75845cc4a4cc5f102629c84f0dade6b2a8d56f".to_string(),
        },
        network_type,
    );

    if let Err(err) = &lock_transaction {
        panic!("{}", err)
    }

    let sig_transaction = account.sign(lock_transaction.unwrap(), &generation_hash);

    let sig_tx = match &sig_transaction {
        Ok(sig) => sig,
        Err(err) => panic!("{}", err),
    };

    println!("Singer: \t{}", account.public_key_string());
    println!("Hash: \t\t{}", sig_tx.get_hash());

    let response = client.transaction_api().announce(&sig_tx).await;

    match response {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{}", err),
    }
}
