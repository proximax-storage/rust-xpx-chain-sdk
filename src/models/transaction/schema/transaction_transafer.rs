use super::{
    ArrayAttribute,
    ScalarAttribute,
    Schema,
    SchemaAttribute,
    SIZEOF_BYTE,
    SIZEOF_INT,
    SIZEOF_SHORT,
    table_attribute::TableAttribute,
    TableArrayAttribute,
};
use super::schema_common_definition::schema_common_definition;

pub fn transfer_transaction_schema() -> Schema {
    let mut schema_definition = schema_common_definition();

    let mut transfer_schema_definition: Vec<Box<dyn SchemaAttribute>> = vec![
        Box::new(ArrayAttribute::new("recipient", SIZEOF_BYTE)),
        Box::new(ScalarAttribute::new("messageSize", SIZEOF_SHORT)),
        Box::new(ScalarAttribute::new("numMosaics", SIZEOF_BYTE)),
        Box::new(TableAttribute::new(
            "message",
            vec![
                Box::new(ScalarAttribute::new("type", SIZEOF_BYTE)),
                Box::new(ArrayAttribute::new("payload", SIZEOF_BYTE)),
            ],
        )
        ),
        Box::new(TableArrayAttribute::new(
            "mosaics",
            vec![
                Box::new(ArrayAttribute::new("id", SIZEOF_INT)),
                Box::new(ArrayAttribute::new("amount", SIZEOF_INT)),
            ],
        )
        )
    ];

    schema_definition.append(&mut transfer_schema_definition);

    Schema::new(schema_definition)
}
