#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_sdk::models::account::{Account, Address};
use xpx_chain_sdk::models::message::PlainMessage;
use xpx_chain_sdk::models::mosaic::Mosaic;
use xpx_chain_sdk::models::network::PUBLIC_TEST;
use xpx_chain_sdk::models::transaction::TransferTransaction;
use xpx_chain_sdk::models::transaction::deadline::Deadline;

fn main() {
    let network_type = PUBLIC_TEST;

    // Deadline default 1 hour
    let deadline = Deadline::default();

    let private_key = "3B49BF0A08BB7528E54BB803BEEE0D935B2C800364917B6EFF331368A4232FD5";

    let account = Account::from_private_key(private_key, network_type).unwrap();

    let recipient = Address::from_raw("VAWOEOWTABXR7O3ZAK2XNA5GIBNE6PZIXDAFDWBU").unwrap();

    let message = PlainMessage::new("ProximaX Limited");

    let transfer_transaction = TransferTransaction::new(
        deadline,
        recipient,
        vec![Mosaic::xpx(10)],
        Box::new(message),
        network_type,
    );

    match transfer_transaction {
        Ok(transfer) => {
            let sig_transaction = account.sign_transaction(Box::new(transfer), "".to_owned());
            match sig_transaction {
                Ok(sig) => println!("{}", sig),
                Err(err) => eprintln!("{:?}", err),
            }
        }
        Err(err) => eprintln!("{:?}", err),
    }
}
