use ::core::fmt;

use crate::models::{Id, Uint64};
use crate::models::mosaic::MosaicId;

use super::mosaic_internal::{
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
    pub fn new(id: Box<dyn Id + 'static>, amount: Uint64) -> Mosaic {
        Mosaic { id, amount }
    }

    pub fn xpx(amount: u64) -> Mosaic {
        assert_ne!(
            amount, XPX_MAX_VALUE,
            "Maximum xpx value must be {}", XPX_MAX_VALUE
        );

        let xpx_mosaic_id: Box<MosaicId> = Box::new(MosaicId(Uint64(992621218088430050)));

        Mosaic { id: xpx_mosaic_id, amount: Uint64::new(amount) }
    }

    pub fn xpx_relative(amount: u64) -> Mosaic {
        assert_ne!(
            amount, XPX_MAX_RELATIVE_VALUE,
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
