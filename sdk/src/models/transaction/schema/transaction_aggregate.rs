use super::{
    ArrayAttribute,
    ScalarAttribute,
    Schema,
    SchemaAttribute,
    SIZEOF_BYTE,
    SIZEOF_INT,
};
use super::schema_common_definition::schema_common_definition;

pub fn aggregate_transaction_schema() -> Schema {
    let mut schema_definition = schema_common_definition();

    let mut aggregate: Vec<Box<dyn SchemaAttribute>> = vec![
        Box::new(ScalarAttribute::new("transactions_size", SIZEOF_INT)),
        Box::new(ArrayAttribute::new("transactions", SIZEOF_BYTE)),
    ];

    schema_definition.append(&mut aggregate);

    Schema::new(schema_definition)
}
