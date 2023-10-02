/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use super::{
    ArrayAttribute, Schema, schema_common_definition::schema_common_definition, SchemaAttribute,
    SIZEOF_BYTE, SIZEOF_INT,
};

pub fn modify_metadata_v2_transaction_schema() -> Schema {
    let mut schema_definition = schema_common_definition();

    let mut alias_definition: Vec<Box<dyn SchemaAttribute>> = vec![
        Box::new(ArrayAttribute::new("target_key", SIZEOF_BYTE)),
        Box::new(ArrayAttribute::new("scoped_metadata_key", SIZEOF_INT)),
        Box::new(ArrayAttribute::new("target_id", SIZEOF_BYTE)),
        Box::new(ArrayAttribute::new("value_size_delta", SIZEOF_BYTE)),
        Box::new(ArrayAttribute::new("value_size", SIZEOF_BYTE)),
        Box::new(ArrayAttribute::new("value", SIZEOF_BYTE)),
    ];

    schema_definition.append(&mut alias_definition);

    Schema::new(schema_definition)
}
