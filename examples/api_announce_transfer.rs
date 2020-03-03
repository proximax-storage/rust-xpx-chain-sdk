#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::Client;

use xpx_chain_sdk::apis::sirius_client::SiriusClient;
use xpx_chain_sdk::models::{
    account::{Account, Address},
    message::PlainMessage,
    mosaic::Mosaic,
    network::PUBLIC_TEST,
    transaction::{Deadline, TransferTransaction}
};

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

    let recipient = Address::from_raw("VC4A3Z6ALFGJPYAGDK2CNE2JAXOMQKILYBVNLQFS").unwrap();

    let message = PlainMessage::new("Transfer From ProximaX Rust SDK");

    let transfer_transaction = TransferTransaction::new(
        deadline,
        recipient,
        vec![Mosaic::xpx(1)],
        message,
        network_type,
    );

    let transfer_tx = match &transfer_transaction {
        Ok(transfer) => transfer,
        Err(err) => panic!("{}", err),
    };

    let sig_transaction = account.sign(
        transfer_tx, &generation_hash);

    let sig_tx = match &sig_transaction {
        Ok(sig) => sig,
        Err(err) => panic!("{}", err),
    };

    println!("Singer: \t{}", account.public_account.public_key.to_uppercase());
    println!("Hash: \t\t{}", sig_tx.clone().hash.to_uppercase());

    let response = client.transaction.announce_transaction(&sig_tx).await;

    match response {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{:?}", err),
    }
}
