#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_sdk::models::message::{MessageType, PlainMessage};

fn main() {
    let message_type = MessageType::PlainMessageType;

    println!("Message_type is: {}\n", message_type);

    let plain_message =  PlainMessage::new("ProximaX Limited");
    println!("Plain_message: {}\n", plain_message);

    let plain_message_empty =  PlainMessage::empty();
    println!("Plain_message_empty: {}\n", plain_message_empty);
}
