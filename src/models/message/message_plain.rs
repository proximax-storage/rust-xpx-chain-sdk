/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::anyhow::Result;
use crate::helpers::{hex_to_utf8, is_hex};

use super::{Message, MessageType};

/// The `PlainMessage` struct defines a plain string.
/// When sending it to the network we transform the payload to hex-string.
///
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlainMessage {
    /// Message type
    #[serde(rename = "type")]
    pub r#type: MessageType,
    /// Message payload, it could be the message plain text.
    pub payload: String,
}

impl PlainMessage {
    /// Create a new `PlainMessage`.
    ///
    pub fn create(message: &str) -> Self {
        Self { r#type: MessageType::PlainMessageType, payload: message.into() }
    }

    pub fn empty() -> Self {
        Self { r#type: MessageType::PlainMessageType, payload: String::new() }
    }

    /// The 00 prefix will be attached to the final payload.
    ///
    pub fn from_payload(payload_hex: &str) -> Result<Self> {
        ensure!(is_hex(payload_hex), "payload_hex it's not hex.");
        let payload = hex_to_utf8(payload_hex);
        Ok(Self { r#type: MessageType::PlainMessageType, payload })
    }

    /// It creates a `PlainMessage` from the given bytes.
    ///
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        ensure!(!bytes.is_empty(), "bytes must not be empty.");
        let payload = hex::encode(bytes);
        Self::from_payload(&payload)
    }
}

#[typetag::serde]
impl Message for PlainMessage {
    fn message_type(&self) -> MessageType {
        self.r#type
    }
    fn payload(&self) -> String {
        self.payload.to_owned()
    }
    fn box_clone(&self) -> Box<dyn Message + 'static> {
        Box::new((*self).clone())
    }
}

impl core::fmt::Display for PlainMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}

/// Creates `PlainMessage` with the default parameters.
impl Default for PlainMessage {
    fn default() -> Self {
        PlainMessage::empty()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::message::{Message, MessageType, PlainMessage};

    #[test]
    fn test_default_plain_message() {
        let plain_message = PlainMessage::default();

        assert_eq!(plain_message.payload, "");
        assert_eq!(plain_message.message_type(), MessageType::PlainMessageType);
    }

    #[test]
    fn test_create_plain_message() {
        let payload = "test-message";

        let plain_message = PlainMessage::create(payload);
        assert_eq!(plain_message.payload(), payload);
        assert_eq!(plain_message.to_dto(), "00746573742D6D657373616765");
    }

    #[test]
    fn test_create_plain_message_with_static_method() {
        let payload = "746573742D6D657373616765";

        let plain_message = PlainMessage::from_payload(payload).unwrap();

        assert_eq!(plain_message.payload, "test-message");
        assert_eq!(plain_message.to_dto(), "00746573742D6D657373616765");
    }
}
