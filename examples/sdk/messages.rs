// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_crypto::ExpandedSecretKey;
use xpx_chain_sdk::account::Account;
use xpx_chain_sdk::network::PUBLIC_TEST;

const PRIVATE_KEY_SENDER: &str = "86258172F90639811F2ABD055747D1E11B55A64B68AED2CEA9A34FBD6C0BE791";

const PRIVATE_KEY_RECIPIENT: &str = "3B49BF0A08BB7528E54BB803BEEE0D935B2C800364917B6EFF331368A4232FD5";

fn main() {
    // let message_type = MessageType::PlainMessageType;
    //
    // println!("Message_type is: {}\n", message_type);
    //
    // let plain_message = PlainMessage::new("ProximaX Limited");
    // println!("Plain_message: {}\n", plain_message);
    //
    // let plain_message_empty = PlainMessage::empty();
    // println!("Plain_message_empty: {}\n", plain_message_empty);

    let message = "PLOMO".as_bytes();

    let sender_account = Account::from_private_key(PRIVATE_KEY_SENDER, PUBLIC_TEST).unwrap();

    let sender_sk_expanded: ExpandedSecretKey = ExpandedSecretKey::from(&sender_account.key_pair.secret);

    let recipient_account = Account::from_private_key(PRIVATE_KEY_RECIPIENT, PUBLIC_TEST).unwrap();

    // let recipient_sk_expanded: ExpandedSecretKey = ExpandedSecretKey::from(&recipient_sk);

    let secure_message = sender_sk_expanded.sign_prehashed(message, &recipient_account.key_pair.public, None);

    println!("{:?}", &secure_message.to_bytes()[..]);
}
