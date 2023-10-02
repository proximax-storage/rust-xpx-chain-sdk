/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::network::NetworkType;

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct NetworkInfo {
    #[serde(rename = "name")]
    pub network_type: NetworkType,
    pub description: String,
}

impl core::fmt::Display for NetworkInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap_or_default())
    }
}
