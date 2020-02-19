use std::fmt;
use std::rc::Rc;

use rand::distributions::uniform::SampleBorrow;

use super::SchemaAttribute;

pub struct Schema {
    pub definition: Vec<Box<dyn SchemaAttribute>>
}

impl Schema {
    pub fn new(definition: Vec<Box<dyn SchemaAttribute>>) -> Self {
        Schema { definition }
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> Vec<u8> {
        let mut result_bytes: Vec<u8> = Vec::new();

        for i in 0..self.definition.len() {
            let mut temp: &Vec<u8> = &self.definition[1].serialize(
                buffer, 4 + (i * 2), buffer[0] as usize,
            );
            let mut temp = Rc::new(temp);
            result_bytes.append(&mut temp.to_vec());
        }
        result_bytes
    }
}
