// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use std::fmt;

use crate::models::message::MessageType;

pub trait Message: Sync + Send + erased_serde::Serialize
    where
        Self: fmt::Debug,
{
    fn message_type(&self) -> &MessageType;

    fn payload_to_bytes(&self) -> &[u8];

    fn box_clone(&self) -> Box<dyn Message>;
}

// implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Message + 'static> {
    fn clone(&self) -> Box<dyn Message + 'static> {
        self.box_clone()
    }
}

serialize_trait_object!(Message);

impl fmt::Display for dyn Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AbsTransaction: {}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}
