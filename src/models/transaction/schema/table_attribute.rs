use std::borrow::Borrow;

use super::{AbstractSchemaAttribute, SchemaAttribute};
use super::schema;

#[derive(Debug, Serialize)]
pub(crate) struct TableAttribute {
    abs_schema_attribute: AbstractSchemaAttribute,
    schema: Vec<Box<dyn SchemaAttribute>>,
}

impl TableAttribute {
    pub fn new(name: &str, schema: Vec<Box<dyn SchemaAttribute>>) -> Self {
        TableAttribute {
            abs_schema_attribute: AbstractSchemaAttribute {
                name: name.parse().unwrap()
            },
            schema
        }
    }
}

impl SchemaAttribute for TableAttribute {
    fn serialize(&mut self, buffer: &mut [u8], position: usize, inner_object_position: usize) -> Vec<u8> {
        let mut abs = &mut self.abs_schema_attribute;

        let mut result_bytes: Vec<u8> = Vec::new();

        let table_start_position: usize = abs.find_object_start_position(inner_object_position, position, buffer);

        for i in 0..self.schema.len() {
            let temp: Vec<u8> = self.schema[i].serialize(
                buffer, 4 + (i * 2), table_start_position,
            );
            result_bytes.append(&mut temp.to_vec());
        }

        result_bytes
    }
}
