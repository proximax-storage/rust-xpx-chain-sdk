use std::fmt;

use crate::models::Uint64;

use super::{MosaicId, MosaicProperties};

/// Contains information about a mosaic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MosaicInfo {
    /// The mosaic ID.
    pub mosaic_id: MosaicId,

    /// The total supply of the mosaic.
    pub supply: Uint64,

    /// The block height the mosaic was created.
    pub height: Uint64,

    /// The account of the owner of this mosaic.
    pub owner: String,

    /// The mosaic revision.
    pub revision: usize,

    /// The mosaic flags.
    properties: MosaicProperties,
}

impl MosaicInfo {
    pub fn new(
        mosaic_id: MosaicId,
        supply: Uint64,
        height: Uint64,
        owner: String,
        revision: usize,
        properties: MosaicProperties,
    ) -> Self {
        MosaicInfo {
            mosaic_id,
            supply,
            height,
            owner,
            revision,
            properties,
        }
    }

    /// Returns the mosaic supply mutability.
    pub fn is_supply_mutable(&self) -> bool {
        self.properties.supply_mutable
    }

    /// Returns the mosaic transferability.
    pub fn is_transferable(&self) -> bool { self.properties.transferable }
}

impl fmt::Display for MosaicInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f, "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
