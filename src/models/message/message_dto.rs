use crate::models::message::{Message, PlainMessage};

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct MessageDto {
    #[serde(rename = "type")]
    _type: u8,
    #[serde(rename = "payload")]
    payload: String,
}

impl MessageDto {
    pub fn to_struct(&self) -> Box<dyn Message> {
        if self._type == 0 {
            let mut plain = PlainMessage::default();
            if self.payload.len() != 0 {
                let b = hex::decode(&self.payload).unwrap();
                plain = PlainMessage::new(&String::from_utf8(b).unwrap());
            }
            return Box::new(plain);
        } else {
            unimplemented!()
        }
    }
}


