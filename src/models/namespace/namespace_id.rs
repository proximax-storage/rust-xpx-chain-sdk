/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::ops::Deref;

use {
    ::std::fmt,
    serde::{Serialize, Serializer},
};

use crate::{
    helpers::has_bits,
    models::{asset_id_model::AssetId, errors_const, Uint64},
    AssetIdType,
};

use super::{generate_namespace_path, NAMESPACE_BIT};

/// The `MosaicId` id structure describes mosaic id.
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Copy)]
pub struct NamespaceId(Uint64);

impl NamespaceId {
    /// Creates a new `MosaicId` from a `Uint64`.
    pub fn new(id: u64) -> NamespaceId {
        assert!(
            id != 0 && has_bits(id, NAMESPACE_BIT),
            errors_const::ERR_WRONG_BIT_NAMESPACE_ID
        );

        NamespaceId(Uint64::new(id))
    }

    /// Creates a new `NamespaceId` from a hex string.
    pub fn from_name(string_name: &str) -> crate::Result<NamespaceId> {
        ensure!(
            !string_name.is_empty(),
            errors_const::ERR_EMPTY_NAMESPACE_NAME
        );

        let list = generate_namespace_path(string_name)?;

        ensure!(!list.is_empty(), errors_const::ERR_INVALID_NAMESPACE_NAME);

        Ok(list[list.len() - 1])
    }

    /// Creates a new `MosaicId` from a pair of 32-bit integers.
    pub fn from_ints(lower: u32, higher: u32) -> NamespaceId {
        NamespaceId(Uint64::from((lower, higher)))
    }
}

impl AssetId for NamespaceId {
    fn to_uint64(&self) -> Uint64 {
        self.0
    }

    fn box_clone(&self) -> Box<dyn AssetId + 'static> {
        Box::new(*self)
    }

    fn get_type(&self) -> AssetIdType {
        AssetIdType::NamespaceIdType
    }
}

impl fmt::Display for NamespaceId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl Serialize for NamespaceId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_hex())
    }
}

impl From<Uint64> for NamespaceId {
    fn from(e: Uint64) -> Self {
        NamespaceId(e)
    }
}

impl From<&str> for NamespaceId {
    fn from(hex: &str) -> Self {
        NamespaceId::from(Uint64::from_hex(hex).unwrap())
    }
}

// Enable `Deref` coercion NamespaceId.
impl Deref for NamespaceId {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
