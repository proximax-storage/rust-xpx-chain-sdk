#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::Client;

use xpx_chain_sdk::account::Account;
use xpx_chain_sdk::mosaic::{MosaicNonce, MosaicProperties};
use xpx_chain_sdk::network::PUBLIC_TEST;
use xpx_chain_sdk::sirius_client::SiriusClient;
use xpx_chain_sdk::transaction::{Deadline, MosaicDefinitionTransaction};
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
    // let deadline = Deadline::new(1, 0, 0);
    let deadline = Deadline::default();

    let account = Account::from_private_key(PRIVATE_KEY, network_type).unwrap();

    let mosaic_definition = MosaicDefinitionTransaction::new(
        deadline,
        MosaicNonce::random(),
        account.to_owned().public_account,
        MosaicProperties::new(
            true, true, 6, Uint64::new(0)
        ).unwrap(),
        network_type);

    if let Err(err) = &mosaic_definition {
        panic!("{}", err)
    }

    let sig_mosaic_definition = account.sign(mosaic_definition.unwrap(), &generation_hash);

    if let Err(err) = &sig_mosaic_definition {
        panic!("{}", err)
    }

    let sig_transaction = &sig_mosaic_definition.unwrap();

    println!("Singer: \t{}", account.public_account.public_key.to_uppercase());
    println!("Hash: \t\t{}", sig_transaction.hash.to_uppercase());

    let response = client.transaction.announce(sig_transaction).await;

    match response {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{:?}", err),
    }
}