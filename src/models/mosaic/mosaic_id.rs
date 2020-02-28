use ::core::fmt;

use serde::{Serialize, Serializer};

use crate::models::{account::PublicAccount,
                    Id,
                    Uint64,
                    utils::utils_hex::is_hex,
};

use super::{generate_mosaic_id, MosaicNonce};

/// The `MosaicId` id structure describes mosaic id.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct MosaicId(pub(crate) Uint64);

impl MosaicId {
    /// Creates a new `MosaicId` from a `Uint64`.
    pub fn new(value: u64) -> MosaicId {
        MosaicId(Uint64::new(value))
    }

    /// Creates a new `MosaicId` from a hex string.
    pub fn from_hex(string_hex: &str) -> crate::Result<MosaicId> {
        ensure!(
            !string_hex.is_empty(),
            "The hex string must not be empty."
         );

        ensure!(
            is_hex(string_hex),
            "Invalid hex string."
            );

        Ok(MosaicId(Uint64::from_hex(string_hex)?))
    }

    /// Creates a new `MosaicId` from a pair of 32-bit integers.
    pub fn from_ints(lower: u32, higher: u32) -> MosaicId {
        MosaicId(Uint64::from_ints(lower, higher))
    }

    /// Creates a new `MosaicId` from a given `mosaic_nonce` and owner's `PublicAccount`.
    pub fn from_nonce_and_owner(nonce: MosaicNonce, owner_public_id: PublicAccount) -> MosaicId {
        let id = generate_mosaic_id(nonce, owner_public_id);
        MosaicId(id)
    }
}

impl Id for MosaicId {
    fn to_bytes(&self) -> [u8; 8] {
        let id = &self.0;
        id.to_bytes()
    }

    fn to_hex(&self) -> String {
        let id = &self.0;
        id.to_hex()
    }

    fn to_id(&self) -> Uint64 {
        self.0
    }

    fn to_int_array(&self) -> [u32; 2] {
        let id = &self.0;
        id.to_int_array()
    }

    fn eq(&self, other: &dyn Id) -> bool {
        &self.to_bytes() == &other.to_bytes()
    }
}

impl fmt::Display for MosaicId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}", self.0)
    }
}

impl Serialize for MosaicId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_hex())
    }
}

impl From<Uint64> for MosaicId {
    fn from(e: Uint64) -> Self {
        MosaicId(e)
    }
}

impl From<u64> for MosaicId {
    fn from(e: u64) -> Self {
        MosaicId(Uint64::new(e))
    }
}