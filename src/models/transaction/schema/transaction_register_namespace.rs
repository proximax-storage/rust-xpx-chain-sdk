/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use super::{
    ArrayAttribute, ScalarAttribute, Schema, schema_common_definition::schema_common_definition,
    SchemaAttribute, SIZEOF_BYTE, SIZEOF_INT,
};

pub fn register_namespace_transaction_schema() -> Schema {
    let mut schema_definition = schema_common_definition();

    let mut register_namespace_definition: Vec<Box<dyn SchemaAttribute>> = vec![
        Box::new(ScalarAttribute::new("namespace_type", SIZEOF_BYTE)),
        Box::new(ArrayAttribute::new("duration_parent_id", SIZEOF_INT)),
        Box::new(ArrayAttribute::new("namespace_id", SIZEOF_INT)),
        Box::new(ScalarAttribute::new("namespace_name_size", SIZEOF_BYTE)),
        Box::new(ArrayAttribute::new("name", SIZEOF_BYTE)),
    ];

    schema_definition.append(&mut register_namespace_definition);

    Schema::new(schema_definition)
}
