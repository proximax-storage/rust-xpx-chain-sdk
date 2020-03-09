#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_sdk::account::{Account, Address, PublicAccount};
use xpx_chain_sdk::network::PUBLIC_TEST;

fn main() {

    let network_type = PUBLIC_TEST;

    let private_key = "3B49BF0A08BB7528E54BB803BEEE0D935B2C800364917B6EFF331368A4232FD5";

    let account = Account::from_private_key(private_key, network_type).unwrap();
    println!("Account From PrivateKey: {}\n", account);

    let public_key = "c6075f6bf21434010aec83d033cf51cd4ce58ed3bb89fea3e91af7004e2bdb61";

    let public_account = PublicAccount::from_public_key(public_key, network_type).unwrap();
    println!("PublicAccount From Public: {}\n", public_account);

    let address = Address::from_public_key(public_key, network_type);
    println!("Address From PublicKey: {}\n", address);

    let message = "ProximaX Limited";

    let sig = account.sign_data(message.as_ref());
    println!("Sig From PrivateKey: {}\n", sig);

    let verify_sign = public_account.verify_sign(message, &sig);
    println!("Verify_sign From PublicKey: {:?}\n", verify_sign);
}
