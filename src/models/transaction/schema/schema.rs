/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use super::SchemaAttribute;

pub struct Schema {
    pub definition: Vec<Box<dyn SchemaAttribute>>,
}

impl Schema {
    pub fn new(definition: Vec<Box<dyn SchemaAttribute>>) -> Self {
        Schema { definition }
    }

    pub fn serialize(&mut self, buffer: &mut [u8]) -> Vec<u8> {
        let mut result_bytes: Vec<u8> = Vec::new();

        for i in 0..self.definition.len() {
            let temp: &[u8] =
                &self.definition[i].serialize(buffer, 4 + (i * 2), buffer[0] as usize);

            result_bytes.append(&mut temp.to_vec());
        }
        result_bytes
    }
}
