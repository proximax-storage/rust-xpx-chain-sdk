/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    serde::{Serialize, Serializer},
    std::{fmt, ops::Deref, str::FromStr},
};

use crate::{
    models::errors_const::ERR_INVALID_DATA_LENGTH,
    utils::{hex_to_vec_u8, is_hex},
};

#[derive(Clone, Copy)]
pub struct Signature([u8; Signature::LENGTH]);

impl Signature {
    /// The length of the Signature in bytes.
    pub const LENGTH: usize = 64;
    /// The length of the Signature in bits.
    pub const LENGTH_IN_BITS: usize = Self::LENGTH * 8;
    /// The length of the Signature in hex string.
    pub const LENGTH_IN_HEX: usize = Self::LENGTH * 2;

    /// Create a new [`Signature`] from a byte array.
    pub fn new(data: [u8; Signature::LENGTH]) -> Self {
        Signature(data)
    }

    pub fn from_slice(src: &[u8]) -> crate::Result<Self> {
        ensure!(
            src.len() == Self::LENGTH,
            "Signature decoding failed due to length mismatch. Signature \
             length: {}, src length: {}",
            Self::LENGTH,
            src.len()
        );
        let mut value = Self::zero();
        value.0.copy_from_slice(src);
        Ok(value)
    }

    /// View this Signature as a byte array.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &*self
    }

    pub fn to_hex(&self) -> String {
        hex::encode_upper(&self.0[..])
    }

    /// Creates a zero-initialized instance.
    pub const fn zero() -> Self {
        Self([0; Self::LENGTH])
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.iter() {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.to_string())
    }
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl FromStr for Signature {
    type Err = failure::Error;

    fn from_str(src: &str) -> crate::Result<Self> {
        ensure!(!src.is_empty(), ERR_INVALID_DATA_LENGTH);

        is_hex(src);

        ensure!(
            src.len() == Self::LENGTH_IN_HEX,
            "Signature decoding failed due to length mismatch. Signature \
             length: {}, src length: {}",
            Self::LENGTH_IN_HEX,
            src.len()
        );

        Ok(Self::from_slice(&hex_to_vec_u8(src))?)
    }
}

// Enable `Deref` coercion Signature.
impl Deref for Signature {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
