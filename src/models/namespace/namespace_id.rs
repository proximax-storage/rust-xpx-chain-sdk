/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::any::Any;
use std::ops::Deref;

use serde::{Deserialize, Deserializer};
use serde::de::Error;

use {
    ::std::fmt,
    anyhow::Result,
    serde::{Serialize, Serializer},
};

use crate::account::{alias_to_recipient, UnresolvedAddress};
use crate::AsUint64;
use crate::errors_const;
use crate::helpers::{has_bits, is_hex};
use crate::mosaic::{AssetIdType, UnresolvedMosaicId};
use crate::network::NetworkType;

use super::{generate_namespace_path, NAMESPACE_BIT};

/// The `MosaicId` id structure describes mosaic id.
#[derive(Default, Debug, Clone, PartialEq, Copy)]
pub struct NamespaceId(u64);

impl NamespaceId {
    /// The length of the `NamespaceId` in bytes.
    ///
    const LENGTH_IN_BYTES: usize = 8;

    /// The length of the `NamespaceId` in hex string.
    ///
    pub const LENGTH_IN_HEX: usize = Self::LENGTH_IN_BYTES * 2;

    /// Creates a new `MosaicId` from a `Uint64`.
    pub fn create(id: u64) -> NamespaceId {
        assert!(
            id != 0 && has_bits(id, NAMESPACE_BIT),
            "{}",
            errors_const::ERR_WRONG_BIT_NAMESPACE_ID
        );

        NamespaceId(id)
    }

    /// Creates a new `NamespaceId` from a hex string.
    pub fn from_name(string_name: &str) -> Result<NamespaceId> {
        ensure!(!string_name.is_empty(), errors_const::ERR_EMPTY_NAMESPACE_NAME);

        let list = generate_namespace_path(string_name)?;

        ensure!(!list.is_empty(), errors_const::ERR_INVALID_NAMESPACE_NAME);

        Ok(list[list.len() - 1])
    }

    /// Create a NamespaceId from its encoded hexadecimal notation.
    ///
    pub fn try_from_hex(hex: &str) -> Result<Self> {
        ensure!(!hex.is_empty(), "The hexString must not be null or empty");
        ensure!(is_hex(hex), "Invalid hex");
        ensure!(hex.len() == Self::LENGTH_IN_HEX, "Invalid size for NamespaceId hex");

        Ok(Self(u64::from_hex(hex)?))
    }

    /// Encoded unresolved address.
    ///
    pub fn encode_unresolved_address(&self, network_type: NetworkType) -> Vec<u8> {
        alias_to_recipient(self.0, network_type)
    }
}

#[typetag::serde]
impl UnresolvedMosaicId for NamespaceId {
    fn to_u64(&self) -> u64 {
        **self
    }
    fn box_clone(&self) -> Box<dyn UnresolvedMosaicId> {
        Box::new((*self).clone())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn get_type(&self) -> AssetIdType {
        AssetIdType::NamespaceIdType
    }
}

#[typetag::serde]
impl UnresolvedAddress for NamespaceId {
    fn recipient_to_string(&self) -> String {
        self.to_hex()
    }

    fn unresolved_address_to_bytes(&self, network_type: NetworkType) -> Vec<u8> {
        self.encode_unresolved_address(network_type)
    }

    fn box_clone(&self) -> Box<dyn UnresolvedAddress + 'static> {
        Box::new((*self).clone())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl fmt::Display for NamespaceId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl Serialize for NamespaceId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_hex().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for NamespaceId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let hex_mosaic_id = <String>::deserialize(deserializer)?;
        NamespaceId::try_from_hex(&hex_mosaic_id).map_err(D::Error::custom)
    }
}

impl From<u64> for NamespaceId {
    fn from(e: u64) -> Self {
        NamespaceId(e)
    }
}

/// Creates a `NamespaceId` from the given low and high bits.
///
impl From<[u32; 2]> for NamespaceId {
    fn from(lo_hi: [u32; 2]) -> Self {
        Self(u64::from_bits(lo_hi[0], lo_hi[1]))
    }
}

// Enable `Deref` coercion NamespaceId.
impl Deref for NamespaceId {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::mosaic::UnresolvedMosaicId;
    use crate::namespace::NamespaceId;

    #[test]
    fn test_should_be_created_from_root_namespace_name() {
        let id = NamespaceId::from_name("prx").unwrap();
        assert_eq!(id.to_dto(), [2339353534, 2976741373]);
        assert_eq!(id.to_hex(), "B16D77FD8B6FB3BE");
    }

    #[test]
    fn test_should_be_created_from_sub_namespace_name() {
        let id = NamespaceId::from_name("prx.xpx").unwrap();
        assert_eq!(id.to_dto(), [2434186742, 3220914849]);
        assert_eq!(id.to_hex(), "BFFB42A19116BDF6");
    }

    #[test]
    fn test_should_be_created_from_id() {
        let id = NamespaceId::from([3646934825, 3576016193]);
        assert_eq!(id.to_dto(), [3646934825, 3576016193]);
    }

    #[test]
    #[should_panic(expected = "Namespace name must not by empty")]
    fn test_cannot_parse_str_with_empty_full_name() {
        NamespaceId::from_name("").unwrap();
    }

    #[test]
    #[should_panic(expected = "The hexString must not be null or empty")]
    fn test_cannot_parse_str_with_empty_hex() {
        NamespaceId::try_from_hex("").unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid hex")]
    fn test_cannot_parse_str_with_invalid_hex() {
        NamespaceId::try_from_hex("12zz34").unwrap();
    }
}
