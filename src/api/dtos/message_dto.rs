/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::{
    helpers::hex_decode,
    message::{Message, PlainMessage},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
                let b = hex_decode(&self.payload);
                let mut raw_string = String::new();
                for d in b.clone() {
                    raw_string.push(char::from(d));
                }
                PlainMessage::create(&raw_string)
            } else {
                PlainMessage::default()
            };

            Box::new(plain)
        } else {
            unimplemented!()
        }
    }
}
