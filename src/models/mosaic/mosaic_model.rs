use core::fmt;

use models::*;

/// The `Account` account structure contains account's `PublicAccount` and private key.
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

        if !::models::account::is_hex(string_hex) {
            return Err(ModelError(InternalError::InvalidHex));
        };

        Ok(MosaicId(Uint64::from_hex(string_hex).unwrap()))
    }

    /// Creates a new `MosaicId` from a pair of 32-bit integers.
    pub fn from_ints(lower: i32, higher: i32) -> MosaicId {
        MosaicId(Uint64::from_ints(lower, higher))
    }

    /// Creates a new `MosaicId` from a given osaicNonce and owner's public account.
    pub fn from_nonce(lower: i32, higher: i32) -> MosaicId {
        MosaicId(Uint64::from_ints(lower, higher))
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

    fn to_int_array(&self) -> [i32; 2] {
        let id = &self.0;
        id.to_int_array()
    }
}

impl fmt::Display for MosaicId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}", self.0)
    }
}


