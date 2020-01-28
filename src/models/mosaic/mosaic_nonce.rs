extern crate hex;

use hex::FromHex;

use models::{InternalError, ModelError, Uint64};
use models::utils::{array_of_u8_to_hex_string, array_of_u8_to_u32, u32_to_array_of_u8};

const NONCE_SIZE: usize = 4;

/// The mosaic nonce structure.
#[derive(Debug, Serialize, Deserialize)]
pub struct MosaicNonce(pub(crate) [u8; NONCE_SIZE]);

impl MosaicNonce {
    /// Creates a new `MosaicNonce` from a `[u8; 4]`.
    pub fn new(nonce: [u8; NONCE_SIZE]) -> MosaicNonce {
        MosaicNonce(nonce)
    }

    /// Creates a new `MosaicNonce` from a hex string.
    pub fn from_hex(string_hex: &str) -> Result<MosaicNonce, ModelError> {
        if string_hex.is_empty() {
            return Err(ModelError(InternalError::HexEmptyError));
        }

        if !::models::account::is_hex(string_hex) {
            return Err(ModelError(InternalError::InvalidHex));
        };

        let decoded = <[u8; NONCE_SIZE]>::from_hex(string_hex).unwrap();

        Ok(MosaicNonce(decoded))
    }

    /// Creates a `MosaicNonce` from a `Uint64` value.
    pub fn from_uint64(uint64: Uint64) -> MosaicNonce {
        let int_array = uint64.to_int_array();
        let lower = int_array[0];
        MosaicNonce(u32_to_array_of_u8(lower))
    }

    /// Converts the `MosaicNonce` to a hex string.
    pub fn to_hex(&self) -> String {
        array_of_u8_to_hex_string(self.0.to_vec())
    }

    /// Converts the `MosaicNonce` to a i32.
    pub fn to_i32(&self) -> i32 {
        array_of_u8_to_u32(self.0)
    }
}
