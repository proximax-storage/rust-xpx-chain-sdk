/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    num_enum::{FromPrimitive, IntoPrimitive},
    serde_repr::*,
};

/// The entity type:
#[derive(
Debug,
Clone,
Copy,
Eq,
PartialEq,
Serialize_repr,
Deserialize_repr,
IntoPrimitive,
FromPrimitive,
)]
#[repr(u16)]
#[serde(rename_all = "lowerCase")]
pub enum TransactionGroupType {
    #[num_enum(default)]
    Confirmed,
    Unconfirmed,
    Partial,
    Failed,
}

impl TransactionGroupType {
    pub fn value(self) -> u16 {
        self.into()
    }

    pub fn to_bytes(self) -> [u8; 2] {
        self.value().to_le_bytes()
    }
}

impl core::fmt::Display for TransactionGroupType {
    fn fmt(&self, e: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(e, "{}", format!("{:?}", &self).to_lowercase())
    }
}
