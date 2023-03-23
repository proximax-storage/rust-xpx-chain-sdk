/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::models::transaction::schema::TableArrayAttribute;

use super::{
    schema_common_definition::schema_common_definition, ArrayAttribute, ScalarAttribute, Schema,
    SchemaAttribute, SIZEOF_BYTE, SIZEOF_INT,
};

pub fn modify_metadata_transaction_schema() -> Schema {
    let mut schema_definition = schema_common_definition();

    let mut alias_definition: Vec<Box<dyn SchemaAttribute>> = vec![
        Box::new(ScalarAttribute::new("metadata_type", SIZEOF_BYTE)),
        Box::new(ArrayAttribute::new("metadata_id", SIZEOF_BYTE)),
        Box::new(TableArrayAttribute::new(
            "modifications",
            vec![
                Box::new(ScalarAttribute::new("size", SIZEOF_INT)),
                Box::new(ScalarAttribute::new("modification_type", SIZEOF_BYTE)),
                Box::new(ScalarAttribute::new("key_size", SIZEOF_BYTE)),
                Box::new(ArrayAttribute::new("value_size", SIZEOF_BYTE)),
                Box::new(ArrayAttribute::new("key", SIZEOF_BYTE)),
                Box::new(ArrayAttribute::new("value", SIZEOF_BYTE)),
            ],
        )),
    ];

    schema_definition.append(&mut alias_definition);

    Schema::new(schema_definition)
}
