/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::fmt;
use crate::helpers::is_hex;

use super::{MessageType, PlainMessage};

/// An abstract message trait that serves as the base of all message types.
///
#[typetag::serde]
pub trait Message: Sync + Send + erased_serde::Serialize
    where
        Self: fmt::Debug,
{
    fn message_type(&self) -> MessageType;

    fn box_clone(&self) -> Box<dyn Message>;

    fn payload(&self) -> String;

    fn payload_to_vec(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let payload_str = self.payload();
        if is_hex(&payload_str) {
            buf.extend(hex::decode(&payload_str).unwrap())
        } else {
            buf.extend(payload_str.as_bytes())
        };

        buf
    }

    fn to_vec(&self) -> Vec<u8> {
        if self.payload().is_empty() {
            return vec![];
        };
        let mut buf = Vec::new();
        buf.extend(self.message_type().to_bytes().to_vec());
        buf.extend(self.payload_to_vec());
        buf
    }

    fn to_dto(&self) -> String {
        use MessageType::*;

        if self.payload().is_empty() {
            return String::new();
        };
        match self.message_type() {
            PlainMessageType | SecureMessageType => {
                let dto = hex::encode((self.message_type().value() as u8).to_be_bytes());
                dto + &hex::encode(&self.payload_to_vec())
            }
            _ => String::new(),
        }
            .to_uppercase()
    }
}

impl dyn Message {
    pub fn empty_message() -> PlainMessage {
        PlainMessage::default()
    }
}

impl Clone for Box<dyn Message + 'static> {
    fn clone(&self) -> Box<dyn Message + 'static> {
        self.box_clone()
    }
}

impl<'a> PartialEq for &'a dyn Message {
    fn eq(&self, other: &Self) -> bool {
        self.to_vec() == other.to_vec()
    }
}

impl<'a> PartialEq for Box<dyn Message + 'static> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl fmt::Display for dyn Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::utf8_to_hex;
    use crate::message::{Message, MessageType, PlainMessage, SecureMessage};

    #[test]
    fn test_should_create_an_plain_message_dto_struct() {
        let message = PlainMessage::create("test");
        assert_eq!(message.to_dto(), format!("00{}", utf8_to_hex("test").to_uppercase()));
    }

    #[test]
    fn test_should_create_an_encrypted_message_dto_struct() {
        let message =
            SecureMessage { r#type: MessageType::SecureMessageType, payload: "test".to_string() };
        assert_eq!(message.to_dto(), format!("01{}", utf8_to_hex("test").to_uppercase()));
    }
}
