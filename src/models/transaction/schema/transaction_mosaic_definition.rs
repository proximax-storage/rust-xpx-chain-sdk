/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use super::{
    schema_common_definition::schema_common_definition, ArrayAttribute, ScalarAttribute, Schema,
    SchemaAttribute, TableArrayAttribute, SIZEOF_BYTE, SIZEOF_INT,
};

pub fn mosaic_definition_transaction_schema() -> Schema {
    let mut schema_definition = schema_common_definition();

    let mut transfer_schema_definition: Vec<Box<dyn SchemaAttribute>> = vec![
        Box::new(ScalarAttribute::new("mosaic_nonce", SIZEOF_INT)),
        Box::new(ArrayAttribute::new("mosaic_id", SIZEOF_INT)),
        Box::new(ScalarAttribute::new("num_optional_properties", SIZEOF_BYTE)),
        Box::new(ScalarAttribute::new("flags", SIZEOF_BYTE)),
        Box::new(ScalarAttribute::new("divisibility", SIZEOF_BYTE)),
        Box::new(TableArrayAttribute::new(
            "modifications",
            vec![
                Box::new(ScalarAttribute::new("mosaic_property_id", SIZEOF_BYTE)),
                Box::new(ArrayAttribute::new("value", SIZEOF_INT)),
            ],
        )),
    ];

    schema_definition.append(&mut transfer_schema_definition);

    Schema::new(schema_definition)
}
