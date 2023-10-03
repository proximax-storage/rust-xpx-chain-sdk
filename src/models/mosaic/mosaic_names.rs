/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::fmt;

use super::MosaicId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MosaicNames {
    pub mosaic_id: MosaicId,
    /// The mosaic linked namespace names.
    pub names: Vec<String>,
}

impl MosaicNames {
    pub fn new(mosaic_id: MosaicId, names: Vec<String>) -> Self {
        Self { mosaic_id, names }
    }
}

impl fmt::Display for MosaicNames {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}
