use super::{AbstractSchemaAttribute, SchemaAttribute};

#[derive(Debug, Serialize)]
pub(crate) struct ScalarAttribute {
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
