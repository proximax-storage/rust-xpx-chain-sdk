// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use ::std::fmt;

pub trait SchemaAttribute: Sync + erased_serde::Serialize
where
    Self: fmt::Debug,
{
    fn serialize(
        &mut self,
        buffer: &mut [u8],
        position: usize,
        inner_object_position: usize,
    ) -> Vec<u8>;
}

serialize_trait_object!(SchemaAttribute);

impl fmt::Display for dyn SchemaAttribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
