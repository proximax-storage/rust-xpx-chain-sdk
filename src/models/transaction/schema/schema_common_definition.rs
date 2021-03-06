/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use super::{
    ArrayAttribute, ScalarAttribute, SchemaAttribute, SIZEOF_BYTE, SIZEOF_INT, SIZEOF_SHORT,
};

pub fn schema_common_definition() -> Vec<Box<dyn SchemaAttribute>> {
    vec![
        Box::new(ScalarAttribute::new("size", SIZEOF_INT)),
        Box::new(ArrayAttribute::new("signature", SIZEOF_BYTE)),
        Box::new(ArrayAttribute::new("signer", SIZEOF_BYTE)),
        Box::new(ScalarAttribute::new("version", SIZEOF_INT)),
        Box::new(ScalarAttribute::new("type", SIZEOF_SHORT)),
        Box::new(ArrayAttribute::new("max_fee", SIZEOF_INT)),
        Box::new(ArrayAttribute::new("deadline", SIZEOF_INT)),
    ]
}
