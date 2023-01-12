/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {num_enum::IntoPrimitive, std::fmt};

/// The type of the message:
/// * 0 - Plain text or unencrypted message.
/// * 1 - Secured text or encrypted message.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Copy, IntoPrimitive)]
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
        self.into()
    }
    pub fn to_bytes(&self) -> [u8; 1] {
        (self.value() as u8).to_le_bytes()
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

/// Creates `MessageType` with the default parameters.
impl Default for MessageType {
    fn default() -> Self {
        Self::PlainMessageType
    }
}

#[cfg(test)]
mod tests {
    use crate::message::MessageType;

    #[test]
    fn test_plain_message_type_is_0x00() {
        assert_eq!(MessageType::PlainMessageType as u8, 0x00);
        assert_eq!(MessageType::PlainMessageType as u8, 0);
    }

    #[test]
    fn test_secure_message_type_is_0x01() {
        assert_eq!(MessageType::SecureMessageType as u8, 0x01);
        assert_eq!(MessageType::SecureMessageType as u8, 1);
    }
}
