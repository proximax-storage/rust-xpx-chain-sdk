use super::{
    ArrayAttribute,
    ScalarAttribute,
    Schema,
    SchemaAttribute,
    TableArrayAttribute,
    SIZEOF_BYTE,
};

use super::schema_common_definition::schema_common_definition;

pub fn modify_multisig_account_transaction_schema() -> Schema {
    let mut schema_definition = schema_common_definition();

    let mut modify_multisig_account_transaction: Vec<Box<dyn SchemaAttribute>> = vec![
        Box::new(ScalarAttribute::new("minRemovalDelta", SIZEOF_BYTE)),
        Box::new(ScalarAttribute::new("minApprovalDelta", SIZEOF_BYTE)),
        Box::new(ScalarAttribute::new("numModifications", SIZEOF_BYTE)),
        Box::new(TableArrayAttribute::new(
            "modification",
            vec![
                Box::new(ScalarAttribute::new("type", SIZEOF_BYTE)),
                Box::new(ArrayAttribute::new("cosignatoryPublicKey", SIZEOF_BYTE)),
            ],
        )
        )
    ];

    schema_definition.append(&mut modify_multisig_account_transaction);

    Schema::new(schema_definition)
}
