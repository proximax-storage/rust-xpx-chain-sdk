#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDto {
    #[serde(rename = "type")]
    pub _type: crate::models::MessageTypeEnum,
    /// The message content in hexadecimal.
    #[serde(rename = "payload")]
    pub payload: String,
}

impl MessageDto {
    pub fn new(_type: crate::models::MessageTypeEnum, payload: String) -> MessageDto {
        MessageDto {
            _type,
            payload,
        }
    }
}


