/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    ::hex::FromHex,
    ::rand::rngs::OsRng,
    ::rand::RngCore,
    ::std::fmt,
    serde::{Serialize, Serializer},
};

use crate::utils::{array_u8_to_u32, is_hex, u32_to_array_u8, vec_u8_to_hex};

const NONCE_SIZE: usize = 4;

/// The mosaic nonce structure.
#[derive(Debug, Clone, Deserialize)]
pub struct MosaicNonce(pub(crate) [u8; NONCE_SIZE]);

impl MosaicNonce {
    /// Creates a new `mosaic_nonce` from a `[u8; 4]`.
    pub fn new(nonce: [u8; NONCE_SIZE]) -> MosaicNonce {
        MosaicNonce(nonce)
    }

    /// Creates a new `mosaic_nonce` from a hex string.
    pub fn from_hex(string_hex: &str) -> crate::Result<MosaicNonce> {
        ensure!(!string_hex.is_empty(), "The hex string must not be empty.");

        ensure!(is_hex(string_hex), "Invalid hex string.");

        let mut decoded = <[u8; NONCE_SIZE]>::from_hex(string_hex).unwrap();

        decoded.reverse();

        Ok(MosaicNonce(decoded))
    }

    /// Creates a random `mosaic_nonce`.
    pub fn random() -> MosaicNonce {
        let mut rng = match OsRng::new() {
            Ok(g) => g,
            Err(e) => panic!("Failed to obtain OS RNG: {}", e),
        };

        let num: u32 = rng.next_u32();

        MosaicNonce(u32_to_array_u8(num))
    }

    /// Converts the `mosaic_nonce` to a hex string.
    pub fn to_hex(&self) -> String {
        vec_u8_to_hex(self.0.to_vec())
    }

    /// Converts the `mosaic_nonce` to a u32.
    pub fn to_u32(&self) -> u32 {
        array_u8_to_u32(self.0)
    }

    /// Converts the `mosaic_nonce` to a array u8.
    pub fn to_array(&self) -> [u8; 4] {
        self.0
    }
}

impl From<u32> for MosaicNonce {
    fn from(e: u32) -> Self {
        MosaicNonce::new(u32_to_array_u8(e))
    }
}

impl fmt::Display for MosaicNonce {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", &self.to_u32())
    }
}

impl Serialize for MosaicNonce {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.to_u32())
    }
}
