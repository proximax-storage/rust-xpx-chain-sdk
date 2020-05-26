// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use super::{
    ArrayAttribute,
    ScalarAttribute,
    Schema,
    SchemaAttribute,
    SIZEOF_BYTE,
    SIZEOF_INT,
};
use super::schema_common_definition::schema_common_definition;

pub fn alias_transaction_schema() -> Schema {
    let mut schema_definition = schema_common_definition();

    let mut alias_definition: Vec<Box<dyn SchemaAttribute>> = vec![
        Box::new(ScalarAttribute::new("action_type", SIZEOF_BYTE)),
        Box::new(ArrayAttribute::new("namespace_id", SIZEOF_INT)),
        Box::new(ArrayAttribute::new("aliasId", SIZEOF_BYTE)),
    ];

    schema_definition.append(&mut alias_definition);

    Schema::new(schema_definition)
}
