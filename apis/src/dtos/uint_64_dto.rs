// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use sdk::Uint64;

#[derive(Clone, Deserialize, Serialize)] // we derive Default in order to use the clear() method in Drop
pub struct Uint64Dto([u32; 2]);

impl Uint64Dto {
    pub fn to_struct(&self) -> Uint64 {
        Uint64::from_ints(self.0[0], self.0[1])
    }
}
