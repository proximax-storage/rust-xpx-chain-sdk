/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use super::{
    ArrayAttribute, ScalarAttribute, Schema, schema_common_definition::schema_common_definition,
    SchemaAttribute, SIZEOF_BYTE, TableArrayAttribute,
};

pub fn modify_multisig_account_transaction_schema() -> Schema {
    let mut schema_definition = schema_common_definition();

    let mut modify_multisig_account_transaction: Vec<Box<dyn SchemaAttribute>> = vec![
        Box::new(ScalarAttribute::new("min_removal_delta", SIZEOF_BYTE)),
        Box::new(ScalarAttribute::new("min_approval_delta", SIZEOF_BYTE)),
        Box::new(ScalarAttribute::new("num_modifications", SIZEOF_BYTE)),
        Box::new(TableArrayAttribute::new(
            "modification",
            vec![
                Box::new(ScalarAttribute::new("type", SIZEOF_BYTE)),
                Box::new(ArrayAttribute::new("cosignatory_publicKey", SIZEOF_BYTE)),
            ],
        )),
    ];

    schema_definition.append(&mut modify_multisig_account_transaction);

    Schema::new(schema_definition)
}
