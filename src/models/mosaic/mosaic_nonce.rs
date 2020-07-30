/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::ops::Deref;

use {
    ::hex::FromHex,
    ::rand::rngs::OsRng,
    ::rand::RngCore,
    ::std::fmt,
    serde::{Serialize, Serializer},
};

use crate::helpers::{array_u8_to_u32, hex_encode, is_hex, u32_to_array_u8};

/// The [`MosaicNonce`] structure.
#[derive(Debug, Clone, Deserialize, Copy)]
pub struct MosaicNonce([u8; MosaicNonce::LENGTH]);

impl MosaicNonce {
    /// The length of the [`MosaicNonce`] in bytes.
    pub const LENGTH: usize = 4;
    /// The length of the [`MosaicNonce`] in bits.
    pub const LENGTH_IN_BITS: usize = Self::LENGTH * 8;
    /// The length of the [`MosaicNonce`] in hex string.
    pub const LENGTH_IN_HEX: usize = Self::LENGTH * 2;

    /// Creates a new [`MosaicNonce`] from a `[u8; 4]`.
    pub fn new(nonce: [u8; Self::LENGTH]) -> MosaicNonce {
        MosaicNonce(nonce)
    }

    /// Creates a new [`MosaicNonce`] from a hex string.
    pub fn from_hex(string_hex: &str) -> crate::Result<MosaicNonce> {
        ensure!(!string_hex.is_empty(), "The hex string must not be empty.");

        ensure!(is_hex(string_hex), "Invalid hex string.");

        let mut decoded = <[u8; Self::LENGTH]>::from_hex(string_hex).unwrap();

        decoded.reverse();

        Ok(MosaicNonce(decoded))
    }

    /// Creates a random [`MosaicNonce`].
    pub fn random() -> MosaicNonce {
        let mut key = [0u8; Self::LENGTH];
        OsRng.fill_bytes(&mut key);

        MosaicNonce(key)
    }

    /// Converts the [`MosaicNonce`] to a hex string.
    pub fn to_hex(&self) -> String {
        hex_encode(&self.0)
    }

    /// Converts the [`MosaicNonce`] to a u32.
    pub fn to_u32(&self) -> u32 {
        array_u8_to_u32(self.0)
    }

    /// Converts the [`MosaicNonce`] to a array u8.
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

// Enable `Deref` coercion MosaicNonce.
impl Deref for MosaicNonce {
    type Target = [u8; Self::LENGTH];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
