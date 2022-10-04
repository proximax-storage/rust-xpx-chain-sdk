use std::time::Instant;
use xpx_chain_sdk::account::Address;
use xpx_chain_sdk::message::PlainMessage;
use xpx_chain_sdk::network::NetworkType;
use xpx_chain_sdk::transaction::{Deadline, TransferTransaction};

fn main() {
    let start = Instant::now();

    let transfer_transaction = TransferTransaction::create(
        Deadline::default(),
        Address::from_raw("VC4A3Z6ALFGJPYAGDK2CNE2JAXOMQKILYBVNLQFS").unwrap(),
        vec![],
        PlainMessage::create("NEM"),
        NetworkType::PublicTestNet,
        None,
    )
    .unwrap();

    // transfer_transaction.embedded_to_bytes();
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
