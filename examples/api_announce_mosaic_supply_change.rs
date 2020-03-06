#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::Client;

use xpx_chain_sdk::account::Account;
use xpx_chain_sdk::mosaic::{MosaicId, MosaicSupplyType};
use xpx_chain_sdk::network::PUBLIC_TEST;
use xpx_chain_sdk::sirius_client::SiriusClient;
use xpx_chain_sdk::transaction::{Deadline, MosaicSupplyChangeTransaction};
use xpx_chain_sdk::Uint64;

#[tokio::main]
async fn main() {
    let node = "http://bctestnet3.brimstone.xpxsirius.io:3000";

    let client = SiriusClient::new(node, Client::new());

    let generation_hash = client.generation_hash().await;

    // let network_type = client.network_type().await;
    let network_type = PUBLIC_TEST;

    // Deadline default 1 hour
    // let deadline = Deadline::new(1, 0, 0);
    let deadline = Deadline::default();

    let private_key = "5D3E959EB0CD69CC1DB6E9C62CB81EC52747AB56FA740CF18AACB5003429AD2E";

    let account = Account::from_private_key(private_key, network_type).unwrap();

    let mosaic_supply = MosaicSupplyChangeTransaction::new(
        deadline,
        MosaicSupplyType::Increase,
        MosaicId::from_hex("389B57CFE6FB5394").unwrap(),
        Uint64::new(100000),
        network_type,
    );

    if let Err(err) = &mosaic_supply {
        panic!("{}", err)
    }

    let sig_mosaic_supply = account.sign(&mosaic_supply.unwrap(), &generation_hash);

    if let Err(err) = &sig_mosaic_supply {
        panic!("{}", err)
    }

    let sig_transaction = &sig_mosaic_supply.unwrap();

    println!("Singer: \t{}", account.public_account.public_key.to_uppercase());
    println!("Hash: \t\t{}", sig_transaction.hash.to_uppercase());

    let response = client.transaction.announce_transaction(&sig_transaction).await;

    match response {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{:?}", err),
    }
}
