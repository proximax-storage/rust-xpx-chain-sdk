#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_sdk::models::account::{Account, Address};
use xpx_chain_sdk::models::message::PlainMessage;
use xpx_chain_sdk::models::mosaic::Mosaic;
use xpx_chain_sdk::models::network::PUBLIC_TEST;
use xpx_chain_sdk::models::transaction::TransferTransaction;
use xpx_chain_sdk::models::transaction::deadline::Deadline;
use xpx_chain_sdk::apis::sirius_client::SiriusClient;
use hyper::Client;

#[tokio::main]
async fn main() {

    let node = "http://bctestnetswap.xpxsirius.io:3000";

    let client = SiriusClient::new(node, Client::new());

    let network_type = PUBLIC_TEST;

    // Deadline default 1 hour
    let deadline = Deadline::default();

    let private_key = "5D3E959EB0CD69CC1DB6E9C62CB81EC52747AB56FA740CF18AACB5003429AD2E";

    let account = Account::from_private_key(private_key, network_type).unwrap();

    let recipient = Address::from_raw("VC4A3Z6ALFGJPYAGDK2CNE2JAXOMQKILYBVNLQFS").unwrap();

    let message = PlainMessage::new("Transfer From ProximaX Rust SDK");

    let  transfer_transaction = TransferTransaction::new(
        deadline,
        recipient,
        vec![Mosaic::xpx(1)],
        Box::new(message),
        network_type,
    );

    let transfer_tx = loop {
        match &transfer_transaction {
            Ok(_transfer) => break _transfer,
            Err(_e) => eprintln!("{:?}", _e),
        }
    };

    let sig_transaction = account.sign(
        transfer_tx, "56D112C98F7A7E34D1AEDC4BD01BC06CA2276DD546A93E36690B785E82439CA9".to_owned());

    let sig_tx = loop {
        match  &sig_transaction {
            Ok(sig) => break sig ,
            Err(err) => eprintln!("SIG_ERROR: {:?}", err),
        }
    };

    println!("Singer: \t{}", account.public_account.public_key.to_uppercase());
    println!("Hash: \t\t{}", sig_tx.clone().hash.to_uppercase());

    let response = client.transaction.announce_transaction(&sig_tx).await;

    match response {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("RESP_ERROR: {:?}", err),
    }
}
