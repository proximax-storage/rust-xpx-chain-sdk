/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use super::{
    ArrayAttribute, ScalarAttribute, Schema, schema_common_definition::schema_common_definition,
    SchemaAttribute, SIZEOF_BYTE, TableArrayAttribute,
};

pub fn account_property_transaction_schema() -> Schema {
    let mut schema_definition = schema_common_definition();

    let mut transfer_schema_definition: Vec<Box<dyn SchemaAttribute>> = vec![
        Box::new(ScalarAttribute::new("property_type", SIZEOF_BYTE)),
        Box::new(ScalarAttribute::new("modification_count", SIZEOF_BYTE)),
        Box::new(TableArrayAttribute::new(
            "modifications",
            vec![
                Box::new(ScalarAttribute::new("modificationType", SIZEOF_BYTE)),
                Box::new(ArrayAttribute::new("value", SIZEOF_BYTE)),
            ],
        )),
    ];

    schema_definition.append(&mut transfer_schema_definition);

    Schema::new(schema_definition)
}
