// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use xpx_chain_sdk::account::{Account, Address, PublicAccount};
use xpx_chain_sdk::network::PUBLIC_TEST;
use xpx_chain_sdk::transaction::EntityTypeEnum;

fn main() {
    let network_type = PUBLIC_TEST;

    let private_key = "461CE489BF9588BE3B9670B5CB19C8936946DBE8DC29CF06338133DEE64FC49B";

    // let account_one = Address::from_raw("XBSDJJ5Y5NY7CFN6KCUFW7572K4R57JPV7DEOXI7").unwrap();
    // println!("New Address From PrivateKey: {}\n", account_one);

    let account_two = Account::from_private_key(private_key, network_type).unwrap();
    println!("Account From PrivateKey: {}\n", account_two);
}
