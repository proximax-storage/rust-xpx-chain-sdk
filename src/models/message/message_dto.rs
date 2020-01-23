/// MessageTypeEnum : The type of the message: * 0 - Regular message.
/// The type of the message: * 0 - Regular message.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageTypeEnum {
    #[serde(rename = "0")]
    _0,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDto {
    #[serde(rename = "type")]
    pub _type: crate::models::message::MessageTypeEnum,
    /// The message content in hexadecimal.
    #[serde(rename = "payload")]
    pub payload: String,
}

impl MessageDto {
    pub fn new(_type: crate::models::message::MessageTypeEnum, payload: String) -> MessageDto {
        MessageDto {
            _type,
            payload,
        }
    }
}


