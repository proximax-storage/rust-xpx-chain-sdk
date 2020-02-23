use crate::models::message::{Message, PlainMessage};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageDto {
    #[serde(rename = "type")]
    _type: u8,
    #[serde(rename = "payload")]
    payload: String,
}

impl MessageDto {
    pub fn to_struct(&self) -> Box<dyn Message> {
        if self._type == 0 {
            return Box::new(PlainMessage::new(&self.payload));
        }else {
            unimplemented!()
        }
    }
}


