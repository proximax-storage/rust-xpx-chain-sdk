/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::any::Any;

use serde::{Deserialize, Deserializer, Serializer};
use serde::de::Error;

use {
    ::std::{fmt, ops::Deref},
    anyhow::Result,
    serde::Serialize,
};

use crate::account::PublicAccount;
use crate::models::uint64::AsUint64;
use crate::mosaic::{AssetIdType, UnresolvedMosaicId};

use super::{generate_mosaic_id, MosaicNonce};

/// The `MosaicId` structure describes mosaic id.
///
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct MosaicId(pub(crate) u64);

impl MosaicId {
    /// The length of the `MosaicId` in bytes.
    ///
    const LENGTH_IN_BYTES: usize = 8;

    /// The length of the `MosaicId` in hex string.
    ///
    pub const LENGTH_IN_HEX: usize = Self::LENGTH_IN_BYTES * 2;

    /// Creates a new `MosaicId` from a hex string.
    ///
    pub fn try_from_hex(hex: &str) -> Result<Self> {
        ensure!(hex.len() == Self::LENGTH_IN_HEX, "Invalid size for MosaicId hex");

        Ok(Self(u64::from_hex(hex)?))
    }

    /// Create a `MosaicId` for given `MosaicNonce` MosaicNonce and owner `Address`.
    ///
    pub fn create_from_nonce(nonce: MosaicNonce, owner: PublicAccount) -> Self {
        generate_mosaic_id(nonce, owner)
    }

    pub fn to_dto(self) -> [u32; 2] {
        (*self).to_dto()
    }
}

impl Serialize for MosaicId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_hex().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MosaicId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let hex_mosaic_id = <String>::deserialize(deserializer)?;
        MosaicId::try_from_hex(&hex_mosaic_id).map_err(D::Error::custom)
    }
}

#[typetag::serde]
impl UnresolvedMosaicId for MosaicId {
    fn to_u64(&self) -> u64 {
        self.0
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
        AssetIdType::MosaicIdType
    }
}

impl fmt::Display for MosaicId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

/// Creates a `MosaicId` from the given u64.
///
impl From<u64> for MosaicId {
    fn from(e: u64) -> Self {
        MosaicId(e)
    }
}

/// Creates a `MosaicId` from the given low and high bits.
///
impl From<[u32; 2]> for MosaicId {
    fn from(lo_hi: [u32; 2]) -> Self {
        Self(u64::from_bits(lo_hi[0], lo_hi[1]))
    }
}

impl Deref for MosaicId {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::account::PublicAccount;
    use crate::mosaic::{MosaicId, MosaicNonce, UnresolvedMosaicId};
    use crate::network::NetworkType;

    const PUBLIC_KEY: &str = "b4f12e7c9f6946091e2cb8b6d3a12b50d17ccbbf646386ea27ce2946a7423dcf";

    #[test]
    fn test_should_be_created_from_hex() {
        let mosaic_id = MosaicId::try_from_hex("85BBEA6CC462B244").unwrap();
        assert_eq!(mosaic_id.to_dto(), [3294802500, 2243684972]);
    }

    #[test]
    #[should_panic(expected = "Invalid size for MosaicId hex")]
    fn test_should_return_panic_invalid_size() {
        MosaicId::try_from_hex("85BBEA6CC462B24499").unwrap();
    }

    #[test]
    fn test_should_create_given_nonce_and_owner() {
        let owner = PublicAccount::from_public_key(PUBLIC_KEY, NetworkType::PrivateTest).unwrap();
        let nonce = MosaicNonce::from(0);

        let mosaic_id = MosaicId::create_from_nonce(nonce, owner);
        assert_eq!(mosaic_id.to_dto(), [481110499, 2378596286]);
        assert_eq!(mosaic_id.to_hex(), "8DC67FBE1CAD29E3");
    }

    #[test]
    fn test_should_create_twice_the_same_given_nonce_and_owner() {
        let owner = PublicAccount::from_public_key(PUBLIC_KEY, NetworkType::PrivateTest).unwrap();
        let nonce = MosaicNonce::from(0);

        let mosaic_id_one = MosaicId::create_from_nonce(nonce, owner);
        let mosaic_id_two = MosaicId::create_from_nonce(nonce, owner);
        assert_eq!(mosaic_id_one, mosaic_id_two);
    }
}
