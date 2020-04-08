use super::{
    ArrayAttribute,
    ScalarAttribute,
    Schema,
    SchemaAttribute,
    SIZEOF_BYTE,
    SIZEOF_INT,
};
use super::schema_common_definition::schema_common_definition;

pub fn mosaic_supply_change_transaction_schema() -> Schema {
    let mut schema_definition = schema_common_definition();

    let mut mosaic_supply_change_definition: Vec<Box<dyn SchemaAttribute>> = vec![
        Box::new(ArrayAttribute::new("mosaic_id", SIZEOF_INT)),
        Box::new(ScalarAttribute::new("direction", SIZEOF_BYTE)),
        Box::new(ArrayAttribute::new("delta", SIZEOF_INT))
    ];

    schema_definition.append(&mut mosaic_supply_change_definition);

    Schema::new(schema_definition)
}
