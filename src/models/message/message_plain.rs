use crate::models::message::{Message, MessageType, PLAIN_MESSAGE};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PlainMessage {
    message: Message
}

impl PlainMessage {
    pub fn new(payload: &str) -> Self {
        PlainMessage {
            message: Message::new(PLAIN_MESSAGE, payload.to_owned())
        }
    }

    pub fn payload(&self) -> &str {
        &self.message.payload
    }

    pub fn message_type(self) -> MessageType {
        self.message._type
    }

    pub fn empty() -> Self {
        PlainMessage {
            message: Message::new(
                MessageType::PlainMessageType,
                "".to_string())
        }
    }
}

impl core::fmt::Display for PlainMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f, "{}",
            serde_json::to_string_pretty(&self.message).unwrap_or_default()
        )
    }
}
