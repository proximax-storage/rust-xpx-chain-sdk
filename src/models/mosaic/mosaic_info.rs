/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::fmt;

use super::{MosaicId, MosaicProperties};

/// Contains information about a mosaic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MosaicInfo {
    /// The mosaic ID.
    pub mosaic_id: MosaicId,

    /// The total supply of the mosaic.
    pub supply: u64,

    /// The block height the mosaic was created.
    pub height: u64,

    /// The account of the owner of this mosaic.
    pub owner: String,

    /// The mosaic revision.
    pub revision: usize,

    /// The mosaic flags.
    pub(crate) properties: MosaicProperties,
}

impl MosaicInfo {
    /// Returns the mosaic supply mutability.
    pub fn is_supply_mutable(&self) -> bool {
        self.properties.supply_mutable
    }

    /// Returns the mosaic transferability.
    pub fn is_transferable(&self) -> bool {
        self.properties.transferable
    }
}

impl fmt::Display for MosaicInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}
