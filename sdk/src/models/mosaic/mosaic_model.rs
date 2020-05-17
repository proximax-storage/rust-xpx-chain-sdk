use ::core::fmt;

use num_enum::IntoPrimitive;

use crate::models::{asset_id_model::AssetId, Uint64};
use crate::models::errors::ERR_INVALID_MOSAIC_PROPERTY_ID;
use crate::models::mosaic::internally::PRX_XPX_U64;

use super::internally::{XPX_DIVISIBILITY, XPX_MAX_RELATIVE_VALUE, XPX_MAX_VALUE};
use super::MosaicId;

pub const SUPPLY_MUTABLE: u8 = 0x01;
pub const TRANSFERABLE: u8 = 0x02;

/// MosaicPropertyId :
/// The mosaic propery id means:
/// * 0 - MosaicFlags
/// * 1 - Divisibility
/// * 2 - Duration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, IntoPrimitive)]
#[repr(u8)]
pub enum MosaicPropertyId {
    MosaicFlags,
    Divisibility,
    Duration,
}

impl MosaicPropertyId {
    pub fn value(self) -> u8 {
        self.into()
    }
}

impl From<u8> for MosaicPropertyId {
    fn from(id: u8) -> Self {
        assert!(id <= 2, ERR_INVALID_MOSAIC_PROPERTY_ID);
        match id {
            1 => MosaicPropertyId::Divisibility,
            2 => MosaicPropertyId::Duration,
            _ => MosaicPropertyId::MosaicFlags,
        }
    }
}

/// mosaic_supply_type :
/// The supply modification direction:
/// * 0  - Decrease.
/// * 1  - Increase.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, IntoPrimitive)]
#[repr(u8)]
pub enum MosaicSupplyType {
    Decrease,
    Increase,
}

impl MosaicSupplyType {
    pub fn value(self) -> u8 {
        self.into()
    }
}

impl From<u8> for MosaicSupplyType {
    fn from(num: u8) -> Self {
        match num {
            1 => MosaicSupplyType::Increase,
            _ => MosaicSupplyType::Decrease,
        }
    }
}

/// A `Mosaic` describes an instance of a `Mosaic` definition.
/// Mosaics can be transferred by means of a transfer transaction.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Mosaic {
    /// The mosaic id. This can either be of type `MosaicId` or `NamespaceId`.
    pub asset_id: Box<dyn AssetId + 'static>,
    /// The mosaic amount.
    pub amount: Uint64,
}

impl Mosaic {
    /// Creates a new Mosaic with the given `Id` with the given amount.
    ///
    /// The quantity is always given in smallest units for the mosaic. For example, if it has a
    /// divisibility of 3 the quantity is given in millis.
    pub fn new(asset_id: impl AssetId + 'static, amount: Uint64) -> Self {
        Self {
            asset_id: Box::new(asset_id),
            amount,
        }
    }

    pub fn xpx(amount: u64) -> Self {
        assert!(
            amount <= XPX_MAX_VALUE,
            "Maximum xpx value must be {}",
            XPX_MAX_VALUE
        );

        let xpx_mosaic_id = Box::new(MosaicId(Uint64(PRX_XPX_U64)));

        Self {
            asset_id: xpx_mosaic_id,
            amount: Uint64::new(amount),
        }
    }

    pub fn xpx_relative(amount: u64) -> Self {
        assert!(
            amount <= XPX_MAX_RELATIVE_VALUE,
            "Maximum xpx relative value must be {}",
            XPX_MAX_RELATIVE_VALUE
        );

        Mosaic::xpx(amount * XPX_DIVISIBILITY)
    }
}

impl fmt::Display for Mosaic {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct MosaicIds {
    /// The array of mosaic identifiers.
    #[serde(rename = "mosaicIds")]
    pub mosaic_ids: Vec<String>,
}

impl From<Vec<MosaicId>> for MosaicIds {
    fn from(e: Vec<MosaicId>) -> Self {
        let mut ids = MosaicIds::default();
        for m in e {
            ids.mosaic_ids.push(m.to_hex())
        }
        return ids;
    }
}

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
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
