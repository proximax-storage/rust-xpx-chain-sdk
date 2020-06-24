// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use xpx_chain_sdk::account::{Account, Address, PublicAccount};
use xpx_chain_sdk::network::PUBLIC_TEST;
use xpx_chain_sdk::transaction::EntityTypeEnum;

fn main() {
    let network_type = PUBLIC_TEST;

    let private_key = "461CE489BF9588BE3B9670B5CB18C8936946DBE8DC29CF06338133DEE64FC49B";

    let public_key = "2EF33FC9B00483C3AA4AD3344ED368A0053C273B5155B89C80EFBB12925938EC";

    let address_one = Address::from_raw("VDWZ3HVVQOC7Q7GTLGM7YG6WIMCXM52THSABXQW3").unwrap();
    println!("Address from raw: {}\n", address_one);

    let account_one = PublicAccount::from_public_key(public_key, network_type).unwrap();
    println!("PublicAccount From PublicKey: {}\n", account_one);

    let account_two = Account::from_private_key(private_key, network_type).unwrap();
    println!("Account From PrivateKey: {}\n", account_two);
}
