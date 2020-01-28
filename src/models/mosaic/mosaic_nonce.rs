extern crate hex;
extern crate rand;
use hex::FromHex;

use models::{InternalError, ModelError, Uint64};
use models::utils::{array_of_u8_to_hex_string, u32_to_array_of_u8, array_of_u8_to_i32};

use self::rand::rngs::OsRng;
use self::rand::RngCore;

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

    /// Creates a random `MosaicNonce`.
    pub fn random() -> MosaicNonce {

        let mut rng = match OsRng::new() {
            Ok(g) => g,
            Err(e) => panic!("Failed to obtain OS RNG: {}", e)
        };

        let num:u32 = rng.next_u32();

        MosaicNonce(u32_to_array_of_u8(num))
    }

    /// Creates a `MosaicNonce` from a `Uint64` value.
    pub fn from_uint64(uint64: Uint64) -> MosaicNonce {
        let int_array = uint64.to_int_array();
        let lower = int_array[0] as u32;
        MosaicNonce(u32_to_array_of_u8(lower))
    }

    /// Converts the `MosaicNonce` to a hex string.
    pub fn to_hex(&self) -> String {
        array_of_u8_to_hex_string(self.0.to_vec())
    }

    /// Converts the `MosaicNonce` to a i32.
    pub fn to_u32(&self) -> u32 {
        array_of_u8_to_i32(self.0) as u32
    }
}
