use ::core::fmt;

use crate::models::{Id, Uint64};

use super::mosaic_internal::{
    XPX_DIVISIBILITY,
    XPX_MAX_RELATIVE_VALUE,
    XPX_MAX_VALUE,
    XPX_MOSAIC_ID
};

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

//    pub fn xpx(amount: u64) -> Mosaic {
//        assert_le!(
//            amount, XPX_MAX_VALUE,
//            "Maximum xpx value must be {}", XPX_MAX_VALUE
//        );
//
//        Mosaic { id: XPX_MOSAIC_ID, amount: Uint64::new(amount) }
//    }

//    pub fn xpx_relative(amount: u64) -> Mosaic<'a> {
//        assert_le!(
//            amount, XPX_MAX_RELATIVE_VALUE,
//            "Maximum xpx relative value must be {}", XPX_MAX_RELATIVE_VALUE
//        );
//
//        Mosaic::xpx(amount * XPX_DIVISIBILITY)
//    }

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
