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

use crate::{account::Address, mosaic::MosaicId, namespace::NamespaceId, AssetId};

/// MetadataTypeEnum :
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
        match num {
            1 => MetadataType::MetadataAddressType,
            2 => MetadataType::MetadataMosaicType,
            3 => MetadataType::MetadataNamespaceType,
            _ => MetadataType::MetadataNone,
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

/// MetadataModificationTypeEnum :
/// The type of the metadata modification:
///* 0 - Add metadata.
///* 1 - Remove metadata.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, IntoPrimitive)]
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
        match num {
            1 => MetadataModificationType::Add,
            2 => MetadataModificationType::Remove,
            _ => MetadataModificationType::NotSupported,
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
    pub r#type: MetadataType,
    pub fields: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMetadataInfo {
    pub info: MetadataInfo,
    pub address: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MosaicMetadataInfo {
    pub info: MetadataInfo,
    pub mosaic_id: MosaicId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamespaceMetadataInfo {
    pub info: MetadataInfo,
    pub namespace_id: NamespaceId,
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
        let mut addresses = vec![];

        let mut accounts = MetadataIds::default();

        for (i, id) in ids.iter().enumerate() {
            let _id = id.trim();

            addresses.push(_id.replace("-", "").to_uppercase());

            if i == ids.len() - 1 && !addresses.is_empty() {
                accounts.ids = Some(addresses.to_owned())
            }
        }
        accounts
    }
}

impl From<Vec<MosaicId>> for MetadataIds {
    #[inline]
    fn from(ids: Vec<MosaicId>) -> Self {
        let mut mosaic_ids = vec![];

        let mut metadata_ids = MetadataIds::default();

        for (i, id) in ids.iter().enumerate() {
            mosaic_ids.push(id.to_hex());

            if i == ids.len() - 1 && !mosaic_ids.is_empty() {
                metadata_ids.ids = Some(mosaic_ids.to_owned())
            }
        }
        metadata_ids
    }
}

impl From<Vec<NamespaceId>> for MetadataIds {
    #[inline]
    fn from(ids: Vec<NamespaceId>) -> Self {
        let mut namespace_ids = vec![];

        let mut metadata_ids = MetadataIds::default();

        for (i, id) in ids.iter().enumerate() {
            namespace_ids.push(id.to_hex());

            if i == ids.len() - 1 && !namespace_ids.is_empty() {
                metadata_ids.ids = Some(namespace_ids.to_owned())
            }
        }
        metadata_ids
    }
}
