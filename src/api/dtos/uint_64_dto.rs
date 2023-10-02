/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use serde_json::Value;

use crate::AsUint64;

#[derive(Debug, Clone, Deserialize, Serialize)] // we derive Default in order to use the clear() method in Drop
pub(crate) struct Uint64Dto(pub(crate) [u32; 2]);

impl Uint64Dto {
    pub fn compact(&self) -> u64 {
        u64::from_bits(self.0[0], self.0[1])
    }

    pub fn from_value(value: Value) -> Self {
        Self(serde_json::from_value(value).unwrap())
    }
}
