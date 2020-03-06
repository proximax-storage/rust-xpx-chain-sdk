use ::core::fmt;

use crate::models::{id_model::Id, Uint64};
use crate::models::mosaic::MosaicId;

use super::internally::{
    XPX_DIVISIBILITY,
    XPX_MAX_RELATIVE_VALUE,
    XPX_MAX_VALUE,
};

pub(crate) const SUPPLY_MUTABLE: u8 = 0x01;
pub(crate) const TRANSFERABLE: u8 = 0x02;

/// MosaicPropertyId :
/// The mosaic propery id means:
/// * 0 - MosaicFlags
/// * 1 - Divisibility
/// * 2 - duration
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum MosaicPropertyId {
    MosaicFlags,
    Divisibility,
    Duration,
}

impl From<u8> for MosaicPropertyId {
    fn from(e: u8) -> Self {
        let mut property_id = MosaicPropertyId::MosaicFlags;
        if e == 1 {
            property_id = MosaicPropertyId::Divisibility;
        } else {
            property_id = MosaicPropertyId::Duration;
        }
        property_id
    }
}

/// mosaic_supply_type :
/// The supply modification direction:
/// * 0  - Decrease.
/// * 1  - Increase.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum MosaicSupplyType { Decrease, Increase }

impl From<u8> for MosaicSupplyType {
    fn from(e: u8) -> Self {
        let mut direction = MosaicSupplyType::Decrease;
        if e != 0 {
            direction = MosaicSupplyType::Increase;
        }
        direction
    }
}

/// A `Mosaic` describes an instance of a `Mosaic` definition.
/// Mosaics can be transferred by means of a transfer transaction.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Mosaic {
    /// The mosaic id. This can either be of type `MosaicId` or `NamespaceId`.
    pub asset_id: Box<dyn Id + 'static>,
    /// The mosaic amount.
    pub amount: Uint64,
}

impl Mosaic {
    /// Creates a new Mosaic with the given `Id` with the given amount.
    ///
    /// The quantity is always given in smallest units for the mosaic. For example, if it has a
    /// divisibility of 3 the quantity is given in millis.
    pub fn new(asset_id: impl Id + 'static, amount: Uint64) -> Self {
        Mosaic { asset_id: Box::new(asset_id), amount }
    }

    pub fn xpx(amount: u64) -> Self {
        assert!(
            amount <= XPX_MAX_VALUE,
            "Maximum xpx value must be {}", XPX_MAX_VALUE
        );

        let xpx_mosaic_id = Box::new(MosaicId(Uint64(13833723942089965046)));

        Mosaic { asset_id: xpx_mosaic_id, amount: Uint64::new(amount) }
    }

    pub fn xpx_relative(amount: u64) -> Self {
        assert!(
            amount <= XPX_MAX_RELATIVE_VALUE,
            "Maximum xpx relative value must be {}", XPX_MAX_RELATIVE_VALUE
        );

        Mosaic::xpx(amount * XPX_DIVISIBILITY)
    }

    fn get_id<'a>(&'a self) -> &'a dyn Id {
        &*self.asset_id
    }
}

impl fmt::Display for Mosaic {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f, "{}",
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MosaicNames {
    pub mosaic_id: MosaicId,
    /// The mosaic linked namespace names.
    pub names: Vec<String>,
}

impl MosaicNames {
    pub fn new(mosaic_id: MosaicId, names: Vec<String>) -> Self {
        MosaicNames { mosaic_id, names }
    }
}

impl fmt::Display for MosaicNames {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f, "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
