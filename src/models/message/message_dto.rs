#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDto {
    #[serde(rename = "type")]
    _type: u8,
    #[serde(rename = "payload")]
    payload: String,
}

impl MessageDto {
    pub fn new(_type: u8, payload: String) -> MessageDto {
        MessageDto {
            _type,
            payload,
        }
    }
}


