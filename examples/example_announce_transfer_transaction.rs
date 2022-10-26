use std::time::Instant;

use xpx_chain_sdk::account::{Account, Address};
use xpx_chain_sdk::api::SiriusClient;
use xpx_chain_sdk::message::PlainMessage;
use xpx_chain_sdk::network::NetworkType;
use xpx_chain_sdk::transaction::{Deadline, TransferTransaction};

#[tokio::main]
async fn main() -> Result<(), xpx_chain_sdk::api::error::Error> {
    let start = Instant::now();

    let node_url = "http://demo-api-3.edlx.io:3000";

    let client = SiriusClient::new(node_url).await?;

    // let network_type = xpx_chain_sdk::network::PUBLIC_TEST;
    let network_type = client.network_type();

    let generation_hash = client.generation_hash();

    let private_key: &str = "26b64cb10f005e5988a36744ca19e20d835ccc7c105aaa5f3b212da593180930";

    let account = Account::from_hex_private_key(private_key, network_type).unwrap();
    let recipient = Account::random(network_type).to_address();

    let transaction = TransferTransaction::create(
        Deadline::default(),
        recipient,
        vec![],
        PlainMessage::create("Sirius chain Rust sdk"),
        network_type,
        None,
    )?;
    // println!("{transaction}");

    let sig_transaction = account.sign_transaction(transaction, generation_hash)?;

    let response = client.transaction_api().announce(&sig_transaction).await?;
    println!("{response}");
    println!("Singer: \t{}", account.public_key_to_hex());
    println!("Hash: \t{:X}", sig_transaction.hash);

    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
    Ok(())
}
