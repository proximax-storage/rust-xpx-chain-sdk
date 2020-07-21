/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use ::std::fmt::{Debug, Display, Formatter, Result};
use std::ops::Deref;

use crate::account::{
    PREFIX_MIJIN, PREFIX_MIJIN_TEST, PREFIX_PRIVATE, PREFIX_PRIVATE_TEST, PREFIX_PUBLIC,
    PREFIX_PUBLIC_TEST,
};
use crate::models::errors_const;

/// MIJIN private network identifier. Decimal value = 96.
pub const MIJIN: NetworkType = NetworkType(0x60);

/// MIJIN_TEST private test network identifier. Decimal value = 144.
pub const MIJIN_TEST: NetworkType = NetworkType(0x90);

/// The PUBLIC network identifier. Decimal value = 184.
pub const PUBLIC: NetworkType = NetworkType(0xb8);

/// The PUBLIC_TEST test network identifier. Decimal value = 168.
pub const PUBLIC_TEST: NetworkType = NetworkType(0xa8);

/// The PRIVATE network identifier. Decimal value = 200.
pub const PRIVATE: NetworkType = NetworkType(0xc8);

/// The PRIVATE_TEST test network identifier. Decimal value = 176.
pub const PRIVATE_TEST: NetworkType = NetworkType(0xb0);

pub const ALIAS_ADDRESS: NetworkType = NetworkType(0x91);

pub const NOT_SUPPORTED_NET: NetworkType = NetworkType(0);

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Copy)] // we derive Default in order to use the clear() method in Drop
pub struct NetworkType(u8);

impl NetworkType {
    pub fn to_str(self) -> &'static str {
        match self {
            MIJIN => "MIJIN",
            MIJIN_TEST => "MIJIN_TEST",
            PUBLIC => "PUBLIC",
            PUBLIC_TEST => "PUBLIC_TEST",
            PRIVATE => "PRIVATE",
            PRIVATE_TEST => "PRIVATE_TEST",
            ALIAS_ADDRESS => "ALIAS_ADDRESS",
            _ => "NOT_SUPPORTED_NET",
        }
    }

    pub fn to_hex(self) -> String {
        format!("{:x?}", self.0)
    }
}

impl Display for NetworkType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl From<u8> for NetworkType {
    fn from(u: u8) -> Self {
        match u {
            0x60 => MIJIN,
            0x90 => MIJIN_TEST,
            0xb8 => PUBLIC,
            0xa8 => PUBLIC_TEST,
            0xc8 => PRIVATE,
            0xb0 => PRIVATE_TEST,
            0x91 => ALIAS_ADDRESS,
            _ => NOT_SUPPORTED_NET,
        }
    }
}

impl From<&str> for NetworkType {
    fn from(s: &str) -> Self {
        assert!(!s.is_empty(), errors_const::ERR_EMPTY_NETWORK_TYPE);

        match s {
            "MIJIN" => MIJIN,
            "MIJIN_TEST" => MIJIN_TEST,
            "PUBLIC" => PUBLIC,
            "PUBLIC_TEST" => PUBLIC_TEST,
            "PRIVATE" => PRIVATE,
            "PRIVATE_TEST" => PRIVATE_TEST,
            "ALIAS_ADDRESS" => ALIAS_ADDRESS,
            _ => NOT_SUPPORTED_NET,
        }
    }
}

impl From<char> for NetworkType {
    fn from(c: char) -> Self {
        match c {
            PREFIX_MIJIN => MIJIN,
            PREFIX_MIJIN_TEST => MIJIN_TEST,
            PREFIX_PUBLIC => PUBLIC,
            PREFIX_PUBLIC_TEST => PUBLIC_TEST,
            PREFIX_PRIVATE => PRIVATE,
            PREFIX_PRIVATE_TEST => PRIVATE_TEST,
            _ => NOT_SUPPORTED_NET,
        }
    }
}

// Enable `Deref` coercion NetworkType.
impl Deref for NetworkType {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
