use core::fmt;

use models::{Id, InternalError, ModelError, Uint64};
use models::account::PublicAccount;
use models::utils::utils_hex::is_hex;

use super::{generate_mosaic_id, MosaicNonce};

/// The `MosaicId` id structure describes mosaic id.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MosaicId(pub(crate) Uint64);

impl MosaicId {
    /// Creates a new `MosaicId` from a `Uint64`.
    pub fn from_uin64(uin64: Uint64) -> MosaicId {
        MosaicId(uin64)
    }

    /// Creates a new `MosaicId` from a hex string.
    pub fn from_hex(string_hex: &str) -> Result<MosaicId, ModelError> {
        if string_hex.is_empty() {
            return Err(ModelError(InternalError::HexEmptyError));
        }

        if !is_hex(string_hex) {
            return Err(ModelError(InternalError::InvalidHex));
        };

        Ok(MosaicId(Uint64::from_hex(string_hex).unwrap()))
    }

    /// Creates a new `MosaicId` from a pair of 32-bit integers.
    pub fn from_ints(lower: u32, higher: u32) -> MosaicId {
        MosaicId(Uint64::from_ints(lower, higher))
    }

    /// Creates a new `MosaicId` from a given `MosaicNonce` and owner's `PublicAccount`.
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

    fn to_int_array(&self) -> [u32; 2] {
        let id = &self.0;
        id.to_int_array()
    }

    fn eq(&self, other: &Id) -> bool {
        &self.to_bytes() == &other.to_bytes()
    }
}

impl fmt::Display for MosaicId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}", self.0)
    }
}
