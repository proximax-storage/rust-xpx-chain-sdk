#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::Client;

use xpx_chain_sdk::account::{Account, Address};
use xpx_chain_sdk::message::PlainMessage;
use xpx_chain_sdk::mosaic::Mosaic;
use xpx_chain_sdk::network::PUBLIC_TEST;
use xpx_chain_sdk::sirius_client::SiriusClient;
use xpx_chain_sdk::transaction::{AggregateTransaction, Deadline, Transaction, TransferTransaction};

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

    let transfer_transaction_a = TransferTransaction::new(
        deadline,
        Address::from_raw("VC4A3Z6ALFGJPYAGDK2CNE2JAXOMQKILYBVNLQFS").unwrap(),
        vec![Mosaic::xpx(15)],
        PlainMessage::new("Transfer A From ProximaX Rust SDK"),
        network_type,
    );
    let mut transfer_a = match transfer_transaction_a {
        Ok(t) => t,
        Err(err) => panic!("{}", err)
    };
    transfer_a.to_aggregate(account.public_account.clone());

    let transfer_transaction_b = TransferTransaction::new(
        deadline,
        Address::from_raw("VC4A3Z6ALFGJPYAGDK2CNE2JAXOMQKILYBVNLQFS").unwrap(),
        vec![Mosaic::xpx(15)],
        PlainMessage::new("Transfer B From ProximaX Rust SDK"),
        network_type,
    );

    let mut transfer_b = match transfer_transaction_b {
        Ok(t) => t,
        Err(err) => panic!("{}", err)
    };
    transfer_b.to_aggregate(account.public_account.clone());

    let aggregate_complete = AggregateTransaction::new_complete(
        deadline,
        vec![Box::new(transfer_a), Box::new(transfer_b)],
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
        Err(err) => eprintln!("{}", err),
    }
}
