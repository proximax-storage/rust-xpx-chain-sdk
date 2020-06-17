// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use super::{Message, MessageType};
use std::borrow::Borrow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureMessage {
    #[serde(rename = "type")]
    _type: MessageType,
    payload: Vec<u8>,
}

impl SecureMessage {
    fn new(payload: Vec<u8>) -> Self {
        SecureMessage {
            _type: MessageType::SecureMessageType,
            payload,
        }
    }

    // pub fn from_plaint_text(payload: &str, sender: SecretKey, recipient: PublicKey) -> Self {
    //
    //     let skp = Keypair::from_private_key(sender);
    //
    //     let rkp = Keypair::from_private_key(sender);
    //
    //     SecureMessage {
    //         _type: MessageType::SecureMessageType,
    //         payload: ,
    //     }
    // }
}

impl Message for SecureMessage {
    fn message_type(&self) -> &MessageType {
        &self._type.borrow()
    }

    fn payload_to_bytes(&self) -> &[u8] {
        &self.payload
    }

    fn box_clone(&self) -> Box<dyn Message + 'static> {
        Box::new((*self).clone())
    }
}

impl core::fmt::Display for SecureMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f, "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}