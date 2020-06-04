// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use sdk::Uint64;

#[derive(Clone, Deserialize, Serialize)] // we derive Default in order to use the clear() method in Drop
pub struct Uint64Dto(pub(crate) [u32; 2]);

impl Uint64Dto {
    pub fn compact(&self) -> Uint64 {
        Uint64::from_ints(self.0[0], self.0[1])
    }

    pub fn as_bytes(&self) -> [u32; 2] {
        self.0
    }

    pub fn as_vec(&self) -> Vec<u32> {
        self.0.to_vec()
    }
}
