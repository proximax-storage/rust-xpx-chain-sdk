// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use super::{AbstractSchemaAttribute, SchemaAttribute};

#[derive(Debug, Serialize)]
pub struct ScalarAttribute {
    abs_schema_attribute: AbstractSchemaAttribute,
    size: usize,
}

impl ScalarAttribute {
    pub fn new(name: &str, size: usize) -> Self {
        ScalarAttribute {
            abs_schema_attribute: AbstractSchemaAttribute {
                name: name.parse().unwrap()
            },
            size,
        }
    }
}

impl SchemaAttribute for ScalarAttribute {
    fn serialize(&mut self, buffer: &mut [u8], position: usize, inner_object_position: usize) -> Vec<u8> {
        let abs = &mut self.abs_schema_attribute;
        abs.find_param(inner_object_position, position, buffer, self.size)
    }
}
