use ::core::fmt;

use crate::models::{Id, Uint64};
use crate::models::mosaic::MosaicId;

use super::internally::{
    XPX_DIVISIBILITY,
    XPX_MAX_RELATIVE_VALUE,
    XPX_MAX_VALUE,
};

pub(crate) const SUPPLY_MUTABLE: u8 = 0x01;
pub(crate) const TRANSFERABLE: u8 = 0x02;

/// A `Mosaic` describes an instance of a `Mosaic` definition.
/// Mosaics can be transferred by means of a transfer transaction.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Mosaic {
    /// The mosaic id. This can either be of type `MosaicId` or `NamespaceId`.
    pub id: Box<dyn Id + 'static>,
    /// The mosaic amount.
    pub amount: Uint64,
}

impl Mosaic {
    /// Creates a new Mosaic with the given `Id` with the given amount.
    ///
    /// The quantity is always given in smallest units for the mosaic. For example, if it has a
    /// divisibility of 3 the quantity is given in millis.
    pub fn new(id: impl Id + 'static, amount: Uint64) -> Self {
        Mosaic { id: Box::new(id), amount }
    }

    pub fn xpx(amount: u64) -> Self {
        assert!(
            amount <= XPX_MAX_VALUE,
            "Maximum xpx value must be {}", XPX_MAX_VALUE
        );

        let xpx_mosaic_id = Box::new(MosaicId(Uint64(13833723942089965046)));

        Mosaic { id: xpx_mosaic_id, amount: Uint64::new(amount) }
    }

    pub fn xpx_relative(amount: u64) -> Self {
        assert!(
            amount <= XPX_MAX_RELATIVE_VALUE,
            "Maximum xpx relative value must be {}", XPX_MAX_RELATIVE_VALUE
        );

        Mosaic::xpx(amount * XPX_DIVISIBILITY)
    }

    fn get_id<'a>(&'a self) -> &'a dyn Id {
        &*self.id
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
