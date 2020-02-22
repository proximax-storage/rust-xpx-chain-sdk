#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MessageDto {
    #[serde(rename = "type")]
    _type: u8,
    #[serde(rename = "payload")]
    payload: String,
}

impl MessageDto {
    pub fn new(_type: u8, payload: String) -> Self {
        MessageDto {
            _type,
            payload,
        }
    }
}


