/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use serde_json::Value;

use {
    ::std::{fmt, ops::Deref},
    serde::{Serialize, Serializer},
};

use crate::{
    models::{account::PublicAccount, asset_id_model::AssetId, Uint64},
    utils::is_hex,
    AssetIdType,
};

use super::{generate_mosaic_id, MosaicNonce};

/// The `MosaicId` id structure describes mosaic id.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Eq, Hash)]
pub struct MosaicId(pub(crate) Uint64);

impl MosaicId {
    /// Creates a new `MosaicId` from a `Uint64`.
    pub fn new(value: u64) -> Self {
        Self(Uint64::new(value))
    }

    /// Creates a new `MosaicId` from a hex string.
    pub fn from_hex(string_hex: &str) -> crate::Result<Self> {
        ensure!(!string_hex.is_empty(), "The hex string must not be empty.");

        ensure!(is_hex(string_hex), "Invalid hex string.");

        Ok(Self(Uint64::from_hex(string_hex)?))
    }

    /// Creates a new `MosaicId` from a pair of 32-bit integers.
    pub fn from_ints(lower: u32, higher: u32) -> Self {
        Self(Uint64::from_ints(lower, higher))
    }

    /// Creates a new `MosaicId` from a pair of 32-bit integers.
    pub fn from_value(value: Value) -> Self {
        Self(Uint64::from_value(value))
    }

    /// Creates a new `MosaicId` from a given `mosaic_nonce` and owner's `PublicAccount`.
    pub fn from_nonce_and_owner(nonce: MosaicNonce, owner_public_id: PublicAccount) -> Self {
        let id = generate_mosaic_id(nonce, owner_public_id);
        Self(id)
    }
}

impl AssetId for MosaicId {
    fn to_uint64(&self) -> Uint64 {
        self.0
    }

    fn box_clone(&self) -> Box<dyn AssetId + 'static> {
        Box::new((*self).clone())
    }

    fn get_type(&self) -> AssetIdType {
        AssetIdType::Mosaic
    }
}

impl fmt::Display for MosaicId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}", self.0)
    }
}

impl Serialize for MosaicId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_hex())
    }
}

impl From<Uint64> for MosaicId {
    fn from(e: Uint64) -> Self {
        MosaicId(e)
    }
}

impl From<u64> for MosaicId {
    fn from(e: u64) -> Self {
        MosaicId(Uint64::new(e))
    }
}

// Enable `Deref` coercion NetworkType.
impl Deref for MosaicId {
    type Target = Uint64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
