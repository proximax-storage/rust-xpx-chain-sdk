use super::{
    ArrayAttribute,
    Schema,
    SchemaAttribute,
    SIZEOF_BYTE,
    SIZEOF_INT,
};
use super::schema_common_definition::schema_common_definition;

pub fn lock_funds_transaction_schema() -> Schema {
    let mut schema_definition = schema_common_definition();

    let mut lock_funds_transaction_definition: Vec<Box<dyn SchemaAttribute>> = vec![
        Box::new(ArrayAttribute::new("mosaicId", SIZEOF_INT)),
        Box::new(ArrayAttribute::new("mosaicAmount", SIZEOF_INT)),
        Box::new(ArrayAttribute::new("duration", SIZEOF_INT)),
        Box::new(ArrayAttribute::new("hash", SIZEOF_BYTE)),
    ];

    schema_definition.append(&mut lock_funds_transaction_definition);

    Schema::new(schema_definition)
}
