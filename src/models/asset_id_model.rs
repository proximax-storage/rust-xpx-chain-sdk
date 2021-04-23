/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use ::core::fmt;

use super::{mosaic::MosaicId, namespace::NamespaceId, Uint64};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum AssetIdType {
    #[serde(rename = "NamespaceId")]
    NamespaceIdType,
    #[serde(rename = "MosaicId")]
    MosaicIdType,
}

impl fmt::Display for AssetIdType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

/// An `trait` identifier used to define mosaic_id and namespace_id.
pub trait AssetId: Send + Sync + erased_serde::Serialize
    where
        Self: fmt::Debug,
{
    fn to_uint64(&self) -> Uint64;

    fn to_u64(&self) -> u64 {
        *self.to_uint64()
    }

    fn to_bytes(&self) -> [u8; 8] {
        self.to_uint64().to_le_bytes()
    }

    fn to_hex(&self) -> String {
        self.to_uint64().to_hex()
    }

    fn to_u32_array(&self) -> [u32; 2] {
        self.to_uint64().to_u32_array()
    }

    fn is_empty(&self) -> bool {
        self.to_uint64().to_bytes().len() == 0
    }

    fn box_clone(&self) -> Box<dyn AssetId>;

    fn get_type(&self) -> AssetIdType;
}

impl dyn AssetId {
    pub fn as_mosaic_id(&self) -> crate::Result<MosaicId> {
        match self.get_type() {
            AssetIdType::MosaicIdType => Ok(MosaicId::from(self.to_uint64())),
            _ => Err(failure::err_msg(format!(
                "Wrong MosaicId type; original AssetId type: {:?}",
                self.get_type()
            ))),
        }
    }

    pub fn as_namespace_id(&self) -> crate::Result<NamespaceId> {
        match self.get_type() {
            AssetIdType::NamespaceIdType => Ok(NamespaceId::from(self.to_uint64())),
            _ => Err(failure::err_msg(format!(
                "Wrong NamespaceId type; original AssetId type: {:?}",
                self.get_type()
            ))),
        }
    }
}

// implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn AssetId + 'static> {
    fn clone(&self) -> Box<dyn AssetId + 'static> {
        self.box_clone()
    }
}

serialize_trait_object!(AssetId);

impl<'a> PartialEq for &'a dyn AssetId {
    fn eq(&self, other: &Self) -> bool {
        self.to_bytes() == other.to_bytes()
    }
}

impl fmt::Display for dyn AssetId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}
