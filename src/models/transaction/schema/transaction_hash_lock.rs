/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use super::{
    ArrayAttribute, Schema, schema_common_definition::schema_common_definition, SchemaAttribute,
    SIZEOF_BYTE, SIZEOF_INT,
};

pub fn lock_funds_transaction_schema() -> Schema {
    let mut schema_definition = schema_common_definition();

    let mut lock_funds_transaction_definition: Vec<Box<dyn SchemaAttribute>> = vec![
        Box::new(ArrayAttribute::new("mosaic_id", SIZEOF_INT)),
        Box::new(ArrayAttribute::new("mosaic_amount", SIZEOF_INT)),
        Box::new(ArrayAttribute::new("duration", SIZEOF_INT)),
        Box::new(ArrayAttribute::new("hash", SIZEOF_BYTE)),
    ];

    schema_definition.append(&mut lock_funds_transaction_definition);

    Schema::new(schema_definition)
}
