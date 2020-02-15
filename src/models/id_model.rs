use ::core::fmt;
use crate::models::Uint64;

/// An `trait` identifier used to define mosaic_id and namespaceId.
pub trait Id:  Sync + erased_serde::Serialize
    where
        Self: fmt::Debug,
{
    fn to_bytes(&self) -> [u8; 8];

    fn to_hex(&self) -> String;

    fn to_uint64(&self) -> Uint64;

    fn to_int_array(&self) -> [u32; 2];

    fn eq(&self, other: &dyn Id) -> bool;
}

serialize_trait_object!(Id);

impl<'a> PartialEq for &'a dyn Id {
    fn eq(&self, other: &Self) -> bool {
        &self.to_bytes() == &other.to_bytes()
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}
