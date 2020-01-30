use models::{Id, Uint64};
use core::fmt;

/// A `Mosaic` describes an instance of a `Mosaic` definition.
/// Mosaics can be transferred by means of a transfer transaction.
//#[derive(Debug)]
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
}

impl fmt::Display for Mosaic<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mosaic( id: {}, amount: {})", &self.id.to_hex(), &self.amount)
    }
}

impl fmt::Debug for Mosaic<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Id({:?}), {:?}", &self.id.to_hex(), &self.amount)
    }
}
