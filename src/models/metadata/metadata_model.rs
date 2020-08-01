/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    downcast_rs::__std::collections::HashMap,
    num_enum::IntoPrimitive,
    serde::{Serialize, Serializer},
};

use crate::models::{
    account::Address, consts::SIZE_SIZE, mosaic::MosaicId, namespace::NamespaceId, AssetId,
};

///The type of the metadata:
///* 1 - Address metadata.
///* 2 - Mosaic metadata.
///* 3 - Namespace metadata.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, IntoPrimitive)]
#[repr(u8)]
pub enum MetadataType {
    MetadataNone,
    MetadataAddressType,
    MetadataMosaicType,
    MetadataNamespaceType,
}

impl MetadataType {
    pub fn value(self) -> u8 {
        self.into()
    }

    pub fn to_bytes(&self) -> [u8; 1] {
        [self.value()]
    }
}

impl From<u8> for MetadataType {
    fn from(num: u8) -> Self {
        use MetadataType::*;
        match num {
            1 => MetadataAddressType,
            2 => MetadataMosaicType,
            3 => MetadataNamespaceType,
            _ => MetadataNone,
        }
    }
}

impl core::fmt::Display for MetadataType {
    fn fmt(&self, e: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(e, "{:?}", &self)
    }
}

impl Serialize for MetadataType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.value())
    }
}

/// The type of the metadata modification:
///* 0 - Add metadata.
///* 1 - Remove metadata.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize, IntoPrimitive)]
#[repr(u8)]
pub enum MetadataModificationType {
    #[serde(rename = "0")]
    Add,
    #[serde(rename = "1")]
    Remove,
    NotSupported,
}

impl MetadataModificationType {
    pub fn value(self) -> u8 {
        self.into()
    }

    pub fn to_bytes(self) -> [u8; 1] {
        [self.value()]
    }
}

impl From<u8> for MetadataModificationType {
    fn from(num: u8) -> Self {
        use MetadataModificationType::*;
        match num {
            0 => Add,
            1 => Remove,
            _ => NotSupported,
        }
    }
}

impl core::fmt::Display for MetadataModificationType {
    fn fmt(&self, e: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(e, "{:?}", &self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataInfo {
    #[serde(rename = "metadataId")]
    pub r#type: MetadataType,
    pub fields: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMetadataInfo {
    pub info: MetadataInfo,
    pub address: Address,
}

impl core::fmt::Display for AddressMetadataInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MosaicMetadataInfo {
    pub info: MetadataInfo,
    pub mosaic_id: MosaicId,
}

impl core::fmt::Display for MosaicMetadataInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamespaceMetadataInfo {
    pub info: MetadataInfo,
    pub namespace_id: NamespaceId,
}

impl core::fmt::Display for NamespaceMetadataInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataIds {
    /// The array of addresses.
    #[serde(rename = "metadataIds", skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
}

impl From<Vec<&str>> for MetadataIds {
    #[inline]
    fn from(ids: Vec<&str>) -> Self {
        let mut metadata_ids = MetadataIds::default();

        let addresses: Vec<String> = ids
            .into_iter()
            .map(|id| id.trim().replace("-", "").to_uppercase())
            .collect();

        if !addresses.is_empty() {
            metadata_ids.ids = Some(addresses);
        }
        metadata_ids
    }
}

impl From<Vec<MosaicId>> for MetadataIds {
    #[inline]
    fn from(ids: Vec<MosaicId>) -> Self {
        let mut metadata_ids = MetadataIds::default();

        let mosaic_ids: Vec<String> = ids.into_iter().map(|id| id.to_hex()).collect();

        if !mosaic_ids.is_empty() {
            metadata_ids.ids = Some(mosaic_ids);
        }
        metadata_ids
    }
}

impl From<Vec<NamespaceId>> for MetadataIds {
    #[inline]
    fn from(ids: Vec<NamespaceId>) -> Self {
        let mut metadata_ids = MetadataIds::default();

        let namespace_ids: Vec<String> = ids.into_iter().map(|id| id.to_hex()).collect();

        if !namespace_ids.is_empty() {
            metadata_ids.ids = Some(namespace_ids);
        }

        metadata_ids
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MetadataModification {
    pub r#type: MetadataModificationType,
    pub key: String,
    pub value: String,
}

impl MetadataModification {
    pub fn new(modification_type: MetadataModificationType, key: &str, value: &str) -> Self {
        Self {
            r#type: modification_type,
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    pub fn size(&self) -> usize {
        SIZE_SIZE
            + 1 /* MetadataModificationType size */
            + 1 /* KeySize size */
            + 2 /* ValueSize size */
            + self.key.len()
            + self.value.len()
    }
}

impl core::fmt::Display for MetadataModification {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
