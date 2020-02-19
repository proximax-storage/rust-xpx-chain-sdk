use std::borrow::Borrow;

use super::{AbstractSchemaAttribute, SchemaAttribute};
use super::schema;

#[derive(Debug, Serialize)]
struct ArrayAttribute {
    abs_schema_attribute: AbstractSchemaAttribute,
    size: usize,
}

impl ArrayAttribute {
    pub fn new(name: String, size: usize) -> Self {
        ArrayAttribute { abs_schema_attribute: AbstractSchemaAttribute { name }, size }
    }
}

impl SchemaAttribute for ArrayAttribute {
    fn serialize(&mut self, buffer: &mut [u8], position: usize, inner_object_position: usize) -> Vec<u8> {
        let mut abs = &mut self.abs_schema_attribute;
        abs.find_vector(inner_object_position, position, buffer, self.size)
    }
}
