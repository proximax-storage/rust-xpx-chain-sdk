#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::Client;

use xpx_chain_sdk::account::{Account, PublicAccount};
use xpx_chain_sdk::message::PlainMessage;
use xpx_chain_sdk::mosaic::Mosaic;
use xpx_chain_sdk::network::PUBLIC_TEST;
use xpx_chain_sdk::sirius_client::SiriusClient;
use xpx_chain_sdk::transaction::{Deadline, TransferTransaction, AggregateTransaction};

const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";

const PRIVATE_KEY: &str = "5D3E959EB0CD69CC1DB6E9C62CB81EC52747AB56FA740CF18AACB5003429AD2E";
const PUBLIC_KEY: &str = "CC2A4AFB1985C90DAF1FBC2B3BA8DB606ACF95FF6683A81C516132B3080FDA0D";

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

    let public_account = PublicAccount::from_public_key(PUBLIC_KEY, network_type).unwrap();

    let ttx_one = TransferTransaction::new(
        deadline,
        account.public_account.address.clone(),
        vec![Mosaic::xpx(15)],
        PlainMessage::new("Transfer A From ProximaX Rust SDK"),
        network_type,
    );
    let mut transfer_one = match ttx_one {
        Ok(t) => t,
        Err(err) => panic!("{}", err)
    };
    transfer_one.to_aggregate(public_account.clone());

    let ttx_two = TransferTransaction::new(
        deadline,
        public_account.address,
        vec![Mosaic::xpx(15)],
        PlainMessage::new("Transfer B From ProximaX Rust SDK"),
        network_type,
    );
    let mut transfer_two = match ttx_two {
        Ok(t) => t,
        Err(err) => panic!("{}", err)
    };
    transfer_two.to_aggregate(account.public_account.clone());

    let aggregate_bonded = AggregateTransaction::new_bonded(
        deadline,
        vec![Box::new(transfer_one), Box::new(transfer_two)],
        network_type
    );

    let sig_transaction = account.sign(
        aggregate_bonded.unwrap(), &generation_hash);

    let sig_tx = match &sig_transaction {
        Ok(sig) => sig,
        Err(err) => panic!("{}", err),
    };

    println!("Singer: \t{}", account.public_account.public_key.to_uppercase());
    println!("Hash: \t\t{}", &sig_tx.hash.to_uppercase());

    let response = client.transaction.announce_partial(&sig_tx).await;

    match response {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{:?}", err),
    }
}
