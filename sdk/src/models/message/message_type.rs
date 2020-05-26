// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use std::fmt;

/// MessageType:
/// The type of the message:
/// * 0 - Plain text or unencrypted message.
/// * 1 - Secured text or encrypted message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
#[repr(u8)]
pub enum MessageType {
    /// Plain text or unencrypted message.
    #[serde(rename = "0")]
    PlainMessageType = 0x00,
    /// Secured text or encrypted message.
    #[serde(rename = "1")]
    SecureMessageType = 0x01,
}

impl MessageType {
    /// Returns a 'MessageType' for the given int value.
    ///
    /// Throws an error when the type is unknown.
    pub fn get_type(value: u8) -> crate::Result<Self> {
        match value {
            0x00 => Ok(MessageType::PlainMessageType),
            0x01 => Ok(MessageType::SecureMessageType),
            _ => bail!("unknown message type")
        }
    }

    pub fn value(&self) -> u8 {
        match &self {
            MessageType::PlainMessageType => 0x00,
            MessageType::SecureMessageType => 0x01,
        }
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MessageType::PlainMessageType => write!(f, "PlainMessageType"),
            MessageType::SecureMessageType => write!(f, "SecureMessageType"),
        }
    }
}
