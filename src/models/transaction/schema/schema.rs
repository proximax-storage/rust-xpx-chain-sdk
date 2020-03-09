use std::rc::Rc;

use super::SchemaAttribute;

pub struct Schema {
    pub definition: Vec<Box<dyn SchemaAttribute>>
}

impl Schema {
    pub fn new(definition: Vec<Box<dyn SchemaAttribute>>) -> Self {
        Schema { definition }
    }

    pub fn serialize(&mut self, buffer: &mut [u8]) -> Vec<u8> {
        let mut result_bytes: Vec<u8> = Vec::new();

        for i in 0..self.definition.len() {
            let temp: &Vec<u8> = &self.definition[i].serialize(
                buffer, 4 + (i * 2), buffer[0] as usize,
            );

            let temp = Rc::new(temp);
            result_bytes.append(&mut temp.to_vec());
        }
        result_bytes
    }
}
