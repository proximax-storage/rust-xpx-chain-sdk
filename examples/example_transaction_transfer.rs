#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_sdk::models::account::{Account, Address};
use xpx_chain_sdk::models::network::PUBLIC_TEST;
use xpx_chain_sdk::models::transaction::deadline::Deadline;
use xpx_chain_sdk::models::transaction::{TransferTransaction, Transaction};
use xpx_chain_sdk::models::mosaic::Mosaic;
use xpx_chain_sdk::models::message::PlainMessage;

fn main() {
    let network_type = PUBLIC_TEST;

    // Deadline default 1 hour
    let deadline = Deadline::default();

    let private_key = "3B49BF0A08BB7528E54BB803BEEE0D935B2C800364917B6EFF331368A4232FD5";

    let _account = Account::from_private_key(private_key, network_type).unwrap();

    let recipient = Address::from_raw("VAWOEOWTABXR7O3ZAK2XNA5GIBNE6PZIXDAFDWBU").unwrap();

    let message = PlainMessage::new("ProximaX Limited");

    let transfer = TransferTransaction::new(
        deadline,
        recipient,
        vec![Mosaic::xpx(10)],
        Box::new(message),
        network_type
    );

    match transfer {
        Ok(mut resp) => {
           resp.to_aggregate(_account.public_account);
            println!("{}", resp.generate_bytes())
        },
        Err(err) => eprintln!("{:?}", err),
    }
}
