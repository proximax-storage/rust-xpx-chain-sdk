use core::fmt;

use models::{Id, InternalError, ModelError, Uint64};

use super::mosaic_internal::{XPX_MAX_VALUE, XPX_MOSAIC_ID, XPX_MAX_RELATIVE_VALUE, XPX_DIVISIBILITY};
use super::{MosaicId};
use serde::export::fmt::format;

/// A `Mosaic` describes an instance of a `Mosaic` definition.
/// Mosaics can be transferred by means of a transfer transaction.
#[derive(Debug, Clone, PartialEq)]
//#[derive(PartialEq)]
pub struct Mosaic<'a> {
    /// The mosaic id. This can either be of type `MosaicId` or `NamespaceId`.
//    #[serde(rename = "ID")]
    pub id: &'a (Id + 'a),
    /// The mosaic amount.
//    #[serde(rename = "amount")]
    pub amount: Uint64,
}

impl<'a> Mosaic<'a> {
    /// Creates a new Mosaic with the given `Id` with the given amount.
    ///
    /// The quantity is always given in smallest units for the mosaic. For example, if it has a
    /// divisibility of 3 the quantity is given in millis.
    pub fn new(id: &'a Id, amount: Uint64) -> Mosaic<'a> {
        Mosaic { id, amount }
    }

    pub fn xpx(amount: u64) -> Mosaic<'a> {
        if amount > XPX_MAX_VALUE {
           let err = format!("Maximum xpx value must be {}", XPX_MAX_VALUE);
            return Err(ModelError::default()).expect(&err);
        }

        Mosaic { id: XPX_MOSAIC_ID, amount: Uint64::new(amount) }
    }

    pub fn xpx_relative(amount: u64) -> Mosaic<'a> {
        if amount > XPX_MAX_RELATIVE_VALUE {
            let err = format!("Maximum xpx relative value must be {}", XPX_MAX_VALUE);
            return Err(ModelError::default()).expect(&err);
        }

        Mosaic::xpx(amount * XPX_DIVISIBILITY)
    }
}

impl fmt::Display for Mosaic<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", &self.id, &self.amount.0)
    }
}
