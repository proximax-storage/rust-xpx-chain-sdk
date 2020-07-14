/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::message::{Message, PlainMessage};

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct MessageDto {
    #[serde(rename = "type")]
    _type: u8,
    #[serde(rename = "payload")]
    payload: String,
}

impl MessageDto {
    pub fn compact(&self) -> Box<dyn Message> {
        if self._type == 0 {
            let plain = if !self.payload.is_empty() {
                let b = hex::decode(&self.payload).unwrap();
                PlainMessage::new(&String::from_utf8(b).unwrap())
            } else {
                PlainMessage::default()
            };

            Box::new(plain)
        } else {
            unimplemented!()
        }
    }
}
