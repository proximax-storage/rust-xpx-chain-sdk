use std::borrow::Borrow;

use crate::models::message::{Message, MessageType, PLAIN_MESSAGE};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PlainMessage {
    _type: MessageType,
    pub payload: String,
}

impl PlainMessage {
    pub fn new(payload: &str) -> Self {
        PlainMessage {
            _type: PLAIN_MESSAGE,
            payload: payload.to_owned(),
        }
    }

    pub fn empty() -> Self {
        PlainMessage {
            _type: MessageType::PlainMessageType,
            payload: "".to_string(),
        }
    }
}

impl Message for PlainMessage {
    fn message_type(&self) -> &MessageType {
        &self._type.borrow()
    }

    fn payload_to_bytes(&self) -> &[u8] {
        self.payload.as_bytes()
    }
}

impl core::fmt::Display for PlainMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f, "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
