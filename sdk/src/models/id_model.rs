use ::core::fmt;

use crate::models::mosaic::MosaicId;
use crate::models::namespace::NamespaceId;
use crate::models::Uint64;

/// An `trait` identifier used to define mosaic_id and namespace_id.
pub trait Id: Send + Sync + erased_serde::Serialize
where
    Self: fmt::Debug,
{
    fn to_uint64(&self) -> Uint64;

    fn to_u64(&self) -> u64 {
        self.to_uint64().to_u64()
    }

    fn to_mosaic_id(&self) -> MosaicId {
        MosaicId::from(self.to_uint64())
    }

    fn to_nemspace_id(&self) -> NamespaceId {
        NamespaceId::from(self.to_uint64())
    }

    fn to_bytes(&self) -> [u8; 8] {
        self.to_uint64().to_bytes()
    }

    fn to_hex(&self) -> String {
        self.to_uint64().to_hex()
    }

    fn to_u32_array(&self) -> [u32; 2] {
        self.to_uint64().to_int_array()
    }

    fn is_empty(&self) -> bool {
        self.to_uint64().to_bytes().len() == 0
    }
}

serialize_trait_object!(Id);

impl<'a> PartialEq for &'a dyn Id {
    fn eq(&self, other: &Self) -> bool {
        &self.to_bytes() == &other.to_bytes()
    }
}

impl fmt::Display for dyn Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}
