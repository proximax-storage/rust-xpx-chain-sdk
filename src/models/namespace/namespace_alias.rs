/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::fmt;

use crate::account::Address;
use crate::mosaic::MosaicId;

// NamespaceAlias contains aliased mosaicId or address and type of alias
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceAlias {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mosaic_id: Option<MosaicId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    pub type_: u8,
}

impl fmt::Display for NamespaceAlias {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}
