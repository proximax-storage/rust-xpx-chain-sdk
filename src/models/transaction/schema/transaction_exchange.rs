/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use super::{
    schema_common_definition::schema_common_definition, ArrayAttribute, ScalarAttribute, Schema,
    SchemaAttribute, TableArrayAttribute, SIZEOF_BYTE, SIZEOF_INT, SIZEOF_SHORT,
};

pub fn add_exchange_offer_transaction_schema() -> Schema {
    let mut schema_definition = schema_common_definition();

    let mut transfer_schema_definition: Vec<Box<dyn SchemaAttribute>> = vec![
        Box::new(ScalarAttribute::new("offers_count", SIZEOF_BYTE)),
        Box::new(TableArrayAttribute::new(
            "offers",
            vec![
                Box::new(ArrayAttribute::new("mosaic_id", SIZEOF_INT)),
                Box::new(ArrayAttribute::new("mosaic_amount", SIZEOF_INT)),
                Box::new(ArrayAttribute::new("cost", SIZEOF_INT)),
                Box::new(ScalarAttribute::new("type", SIZEOF_BYTE)),
                Box::new(ArrayAttribute::new("duration", SIZEOF_INT)),
            ],
        )),
    ];

    schema_definition.append(&mut transfer_schema_definition);

    Schema::new(schema_definition)
}

pub fn exchange_offer_transaction_schema() -> Schema {
    let mut schema_definition = schema_common_definition();

    let mut transfer_schema_definition: Vec<Box<dyn SchemaAttribute>> = vec![
        Box::new(ScalarAttribute::new("offers_count", SIZEOF_BYTE)),
        Box::new(TableArrayAttribute::new(
            "offers",
            vec![
                Box::new(ArrayAttribute::new("mosaic_id", SIZEOF_INT)),
                Box::new(ArrayAttribute::new("mosaic_amount", SIZEOF_INT)),
                Box::new(ArrayAttribute::new("cost", SIZEOF_INT)),
                Box::new(ScalarAttribute::new("type", SIZEOF_BYTE)),
                Box::new(ArrayAttribute::new("owner", SIZEOF_BYTE)),
            ],
        )),
    ];

    schema_definition.append(&mut transfer_schema_definition);

    Schema::new(schema_definition)
}

pub fn remove_exchange_offer_transaction_schema() -> Schema {
    let mut schema_definition = schema_common_definition();

    let mut transfer_schema_definition: Vec<Box<dyn SchemaAttribute>> = vec![
        Box::new(ScalarAttribute::new("offers_count", SIZEOF_BYTE)),
        Box::new(TableArrayAttribute::new(
            "offers",
            vec![
                Box::new(ArrayAttribute::new("mosaic_id", SIZEOF_INT)),
                Box::new(ScalarAttribute::new("type", SIZEOF_BYTE)),
            ],
        )),
    ];

    schema_definition.append(&mut transfer_schema_definition);

    Schema::new(schema_definition)
}
