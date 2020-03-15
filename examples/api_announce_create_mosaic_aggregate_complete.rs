#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::Client;

use xpx_chain_sdk::account::Account;
use xpx_chain_sdk::mosaic::{MosaicNonce, MosaicProperties, MosaicSupplyType};
use xpx_chain_sdk::network::PUBLIC_TEST;
use xpx_chain_sdk::sirius_client::SiriusClient;
use xpx_chain_sdk::transaction::{AggregateTransaction,
                                 Deadline, MosaicDefinitionTransaction,
                                 MosaicSupplyChangeTransaction};
use xpx_chain_sdk::Uint64;

const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";

const PRIVATE_KEY: &str = "5D3E959EB0CD69CC1DB6E9C62CB81EC52747AB56FA740CF18AACB5003429AD2E";

#[tokio::main]
async fn main() {
    let client = SiriusClient::new(NODE_URL, Client::new());

    let generation_hash = client.generation_hash().await;

    // let network_type = client.network_type().await;
    let network_type = PUBLIC_TEST;

    // Deadline default 1 hour
    let deadline = Deadline::default();
    //let deadline = Deadline::new(1, 30, 0);

    let account = Account::from_private_key(PRIVATE_KEY, network_type).unwrap();

    let mosaic_definition = MosaicDefinitionTransaction::new(
        deadline,
        MosaicNonce::random(),
        account.to_owned().public_account,
        MosaicProperties::new(
            true, true, 6, Uint64::new(0)
        ).unwrap(),
        network_type);

    let mut transaction_a = match mosaic_definition {
        Ok(t) => t,
        Err(err) => panic!("{}", err)
    };
    transaction_a.to_aggregate(account.public_account.clone());

    let mosaic_supply = MosaicSupplyChangeTransaction::new(
        deadline,
        MosaicSupplyType::Increase,
        transaction_a.mosaic_id.to_owned(),
        Uint64::new(100000),
        network_type,
    );

    let mut transaction_b = match mosaic_supply {
        Ok(t) => t,
        Err(err) => panic!("{}", err)
    };
    transaction_b.to_aggregate(account.public_account.clone());

    let aggregate_complete = AggregateTransaction::new_complete(
        deadline,
        vec![Box::new(transaction_a), Box::new(transaction_b)],
        network_type
    );

    let sig_transaction = account.sign(
        aggregate_complete.unwrap(), &generation_hash);

    let sig_tx = match &sig_transaction {
        Ok(sig) => sig,
        Err(err) => panic!("{}", err),
    };

    println!("Singer: \t{}", account.public_account.public_key.to_uppercase());
    println!("Hash: \t\t{}", &sig_tx.get_hash().to_uppercase());

    let response = client.transaction.announce(&sig_tx).await;

    match response {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{:?}", err),
    }
}
