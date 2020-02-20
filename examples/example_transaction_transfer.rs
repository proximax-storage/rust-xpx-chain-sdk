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

    let transfer_transaction = TransferTransaction::new(
        deadline,
        recipient,
        vec![Mosaic::xpx_relative(2)],
        Box::new(message),
        network_type,
    );

    match transfer_transaction {
        Ok(transfer) => {
            let sig_transaction = account.sign(
                Box::new(transfer), "56D112C98F7A7E34D1AEDC4BD01BC06CA2276DD546A93E36690B785E82439CA9".to_owned());
            match &sig_transaction {
                Ok(sig) => println!("Hash: {}", sig.clone().hash),
                Err(err) => eprintln!("SIG_ERROR: {:?}", err),
            }

            let response = client.transaction.announce_transaction(
                &sig_transaction.unwrap()).await;
            match response {
                Ok(resp) => println!("{}", resp),
                Err(err) => eprintln!("RESP_ERROR: {:?}", err),
            }
        }
        Err(err) => eprintln!("{:?}", err),
    }
}
