/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use super::{
    schema_common_definition::schema_common_definition, ArrayAttribute, ScalarAttribute, Schema,
    SchemaAttribute, SIZEOF_BYTE, SIZEOF_INT,
};

pub fn mosaic_supply_change_transaction_schema() -> Schema {
    let mut schema_definition = schema_common_definition();

    let mut mosaic_supply_change_definition: Vec<Box<dyn SchemaAttribute>> = vec![
        Box::new(ArrayAttribute::new("mosaic_id", SIZEOF_INT)),
        Box::new(ScalarAttribute::new("direction", SIZEOF_BYTE)),
        Box::new(ArrayAttribute::new("delta", SIZEOF_INT)),
    ];

    schema_definition.append(&mut mosaic_supply_change_definition);

    Schema::new(schema_definition)
}
