/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::fmt;

use crate::account::Address;
use crate::metadata::MetadataV2Type;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataEntry {
    pub version: u8,
    pub composite_hash: String,
    pub source_address: Address,
    pub target_key: String,
    pub scoped_metadata_key: String,
    pub target_id: String,
    pub metadata_type: MetadataV2Type,
    pub value_size: u64,
    pub value: String,
    pub id: String,
}

impl fmt::Display for MetadataEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}
