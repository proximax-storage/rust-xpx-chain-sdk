use crate::models::message::MessageType;

/// An abstract 'Message' class that serves as the base class of all message types.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Message {
    #[serde(rename = "type")]
    pub(crate) _type: MessageType,
    /// The message content in hexadecimal.
    #[serde(rename = "payload")]
    pub(crate) payload: String,
}

impl Message {
    pub(crate) fn new(_type: MessageType, payload: String) -> Self {
        Message {
            _type,
            payload,
        }
    }
}
