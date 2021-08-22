/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::fmt;

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
    UnknownMessageType,
}

impl MessageType {
    pub fn value(self) -> u8 {
        self as u8
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MessageType::*;
        match *self {
            PlainMessageType => write!(f, "PlainMessageType"),
            SecureMessageType => write!(f, "SecureMessageType"),
            UnknownMessageType => write!(f, "UnknownMessageType"),
        }
    }
}

/// Returns a 'MessageType' for the given u8 value.
///
/// Throws an UnknownMessageType when the type is unknown.
impl From<u8> for MessageType {
    fn from(num: u8) -> Self {
        match num {
            0x00 => MessageType::PlainMessageType,
            0x01 => MessageType::SecureMessageType,
            _ => MessageType::UnknownMessageType,
        }
    }
}
