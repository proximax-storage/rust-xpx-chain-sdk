use ::hex::FromHex;
use ::rand::RngCore;
use ::rand::rngs::OsRng;

use crate::models::{
    utils::{
        array_u8_to_u32,
        is_hex, u32_to_array_u8,
        vec_u8_to_hex
    },
};

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
    pub fn from_hex(string_hex: &str) -> crate::Result<MosaicNonce> {
        ensure!(
            !string_hex.is_empty(),
            "The hex string must not be empty."
         );

        ensure!(
            is_hex(string_hex),
            "Invalid hex string."
        );

        let mut decoded = <[u8; NONCE_SIZE]>::from_hex(string_hex).unwrap();

        decoded.reverse();

        Ok(MosaicNonce(decoded))
    }

    /// Creates a random `MosaicNonce`.
    pub fn random() -> MosaicNonce {
        let mut rng = match OsRng::new() {
            Ok(g) => g,
            Err(e) => panic!("Failed to obtain OS RNG: {}", e)
        };

        let num: u32 = rng.next_u32();

        MosaicNonce(u32_to_array_u8(num))
    }

    /// Converts the `MosaicNonce` to a hex string.
    pub fn to_hex(&self) -> String {
        vec_u8_to_hex(self.0.to_vec())
    }

    /// Converts the `MosaicNonce` to a u32.
    pub fn to_u32(&self) -> u32 {
        array_u8_to_u32(self.0)
    }

    /// Converts the `MosaicNonce` to a array u8.
    pub fn to_array(&self) -> [u8; 4] {
        self.0
    }
}
