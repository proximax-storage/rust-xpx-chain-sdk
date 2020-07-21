/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    ::std::{fmt, ops::Deref},
    byteorder::{BigEndian, WriteBytesExt},
    failure::_core::ops::BitAnd,
    serde_json::Value,
};

use crate::models::error::Error;
use crate::{models::Result, utils::SIZE_U64};

/// Represents a 64-bit unsigned integer.
///
/// This class uses Dart's native number type `u64` and has a value check for big integers.
/// `u64` will be translated correctly into JavaScript (supported by dart2js).
/// Value range is 0 through 18446744073709551615.
#[derive(Default, Clone, Copy, PartialEq, Serialize, Deserialize, Eq, Hash)] // we derive Default in order to use the clear() method in Drop
pub struct Uint64(u64);

impl Uint64 {
    pub fn new(u: u64) -> Self {
        Self(u)
    }

    /// Creates a `Uint64` from a pair of u32 integers.
    fn from_u32_array(lower: u32, higher: u32) -> Self {
        let mut buf = [0u8; SIZE_U64];
        buf[..4]
            .as_mut()
            .write_u32::<BigEndian>(higher)
            .expect("Unable to write");
        buf[4..]
            .as_mut()
            .write_u32::<BigEndian>(lower)
            .expect("Unable to write");
        Self::from_bytes(buf)
    }

    /// Creates a `Uint64` from a u8 array.
    pub fn from_bytes(b: [u8; 8]) -> Self {
        Self(u64::from_be_bytes(b))
    }

    /// Creates a `Uint64` from a hex &str.
    pub fn from_hex(hex_code: &str) -> Result<Self> {
        let r: u64 = u64::from_str_radix(&hex_code, 16)?;
        Ok(Self(r))
    }

    /// Creates a `Uint64` from a Value type str.
    pub fn from_value(value: Value) -> Result<Self> {
        Self::from_hex(
            value
                .as_str()
                .ok_or_else(|| Error::from("fail to create Uint64 from Value"))?,
        )
    }

    /// Converts to hex String representation.
    pub fn to_hex(&self) -> String {
        format!("{:X}", &self.0)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    /// Converts to 64-bit byte array.
    pub fn to_bytes(&self) -> [u8; SIZE_U64] {
        self.0.to_le_bytes()
    }

    /// Converts to a pair of i32 integers ([lower, higher]).
    pub fn to_i32_array(&self) -> [u32; 2] {
        let lower = self.0 as u32;
        let higher = (self.0 >> 32) as u32;
        [lower, higher]
    }
}

impl fmt::LowerHex for Uint64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.0) // delegate to u64's implementation
    }
}

impl fmt::UpperHex for Uint64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", self.0) // delegate to u64's implementation
    }
}

impl fmt::Binary for Uint64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}", self.0) // delegate to i32's implementation
    }
}

impl BitAnd for Uint64 {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a & b`
    fn bitand(self, rhs: Self) -> Self::Output {
        Uint64(self.0 & rhs.0)
    }
}

impl fmt::Display for Uint64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for Uint64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Enable `Deref` coercion Uint64.
impl Deref for Uint64 {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u64> for Uint64 {
    fn from(u: u64) -> Self {
        Uint64(u)
    }
}

impl From<(u32, u32)> for Uint64 {
    fn from(lo_hi: (u32, u32)) -> Self {
        Self::from_u32_array(lo_hi.0, lo_hi.1)
    }
}
