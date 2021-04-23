/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    serde::{Serialize, Serializer},
    std::{fmt, ops::Deref, str::FromStr},
};

use crate::api::str_to_hash;

/// Output value of our hash function. Intentionally opaque for safety and modularity.
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord, Deserialize)]
pub struct HashValue([u8; HashValue::LENGTH]);

impl HashValue {
    /// The length of the hash in bytes.
    pub const LENGTH: usize = 32;
    /// The length of the hash in bits.
    pub const LENGTH_IN_BITS: usize = Self::LENGTH * 8;
    /// The length of the hash in hex string.
    pub const LENGTH_IN_HEX: usize = Self::LENGTH * 2;

    /// Create a new [`HashValue`] from a byte array.
    pub fn new(hash: [u8; HashValue::LENGTH]) -> Self {
        Self(hash)
    }

    /// Create from [`HashValue`] from slice (e.g. retrieved from storage).
    pub fn from_slice(src: &[u8]) -> crate::Result<Self> {
        ensure!(
            src.len() == Self::LENGTH,
            "Hash decoding failed due to length mismatch. Hash \
             length: {}, src length: {}",
            Self::LENGTH,
            src.len()
        );
        let mut value = Self::zero();
        value.0.copy_from_slice(src);
        Ok(value)
    }

    pub fn to_hex(&self) -> String {
        hex::encode_upper(&self.0[..])
    }

    /// Creates a zero-initialized instance.
    pub const fn zero() -> Self {
        Self([0; Self::LENGTH])
    }

    pub fn as_bytes(&self) -> &[u8] {
        &*self
    }
}

impl FromStr for HashValue {
    type Err = failure::Error;

    fn from_str(src: &str) -> crate::Result<Self> {
        Self::from_slice(&str_to_hash(src)?)
    }
}

impl fmt::Display for HashValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.iter() {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

impl Serialize for HashValue {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl fmt::Debug for HashValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("HashValue")
            .field(&self.to_hex().to_lowercase())
            .finish()
    }
}

impl fmt::Binary for HashValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.iter() {
            write!(f, "{:08b}", byte)?;
        }
        Ok(())
    }
}

// Enable `Deref` coercion HashValue.
impl Deref for HashValue {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[u8; HashValue::LENGTH]> for HashValue {
    fn as_ref(&self) -> &[u8; HashValue::LENGTH] {
        &self.0
    }
}

impl Default for HashValue {
    fn default() -> Self {
        HashValue::zero()
    }
}
