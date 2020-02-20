use std::borrow::Borrow;

use super::{AbstractSchemaAttribute, SchemaAttribute};
use super::schema;

#[derive(Debug, Serialize)]
pub struct TableArrayAttribute {
    abs_schema_attribute: AbstractSchemaAttribute,
    schema: Vec<Box<dyn SchemaAttribute>>,
}

impl TableArrayAttribute {
    pub fn new(name: &str, schema: Vec<Box<dyn SchemaAttribute>>) -> Self {
        TableArrayAttribute {
            abs_schema_attribute: AbstractSchemaAttribute {
                name: name.parse().unwrap()
            },
            schema
        }
    }
}

impl SchemaAttribute for TableArrayAttribute {
    fn serialize(&mut self, buffer: &mut [u8], position: usize, inner_object_position: usize) -> Vec<u8> {

        let mut abs = &mut self.abs_schema_attribute;

        let mut result_bytes: Vec<u8> = Vec::new();

        let array_length: usize = abs.find_array_length(inner_object_position, position, buffer);

        for i in 0..array_length {
            let start_array_position: usize = abs.find_object_array_element_start_position(
                inner_object_position, position, buffer, i
            );

            for j in 0..self.schema.len() {
                let temp: Vec<u8> = self.schema[j].serialize(
                    buffer, 4 + (j * 2), start_array_position,
                );

                if temp.len() == 1 {
                    result_bytes.push(temp[0]);
                } else {
                    result_bytes.append(&mut temp.to_vec());
                }
            }
        }
        result_bytes
    }
}
