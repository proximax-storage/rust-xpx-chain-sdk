/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use downcast_rs::__std::collections::HashMap;

use crate::{
    api::Uint64Dto,
    models::{
        account::Address,
        metadata::{
            AddressMetadataInfo, MetadataInfo, MetadataType, MosaicMetadataInfo,
            NamespaceMetadataInfo,
        },
        mosaic::MosaicId,
        namespace::NamespaceId,
    },
};

use super::FieldDto;

#[derive(Serialize, Deserialize)]
pub(crate) struct MetadataInfoDto {
    #[serde(rename = "metadataType")]
    r#_type: u8,
    fields: Vec<FieldDto>,
}

impl MetadataInfoDto {
    pub fn compact(&self) -> MetadataInfo {
        let mut fields: HashMap<String, String> = HashMap::new();

        self.fields.iter().for_each(|field| {
            fields.insert(String::from(&field.key), String::from(&field.value));
        });

        MetadataInfo {
            r#type: MetadataType::from(self._type),
            fields,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MetadataModificationDto {
    modification_type: u8,
    /// The key of metadata modification.
    key: String,
    /// The value of metadata modification.
    value: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct AddressMetadataInfoDto {
    #[serde(flatten)]
    metadata: MetadataInfoDto,
    #[serde(rename = "metadataId")]
    address: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct AddressMetadataDto {
    metadata: AddressMetadataInfoDto,
}

impl AddressMetadataInfoDto {
    pub fn compact(&self) -> crate::Result<AddressMetadataInfo> {
        let info = self.metadata.compact();

        let address = if !self.address.is_empty() {
            Address::from_encoded(&self.address)?
        } else {
            Address::default()
        };
        Ok(AddressMetadataInfo { info, address })
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MosaicMetadataInfoDto {
    #[serde(flatten)]
    metadata: MetadataInfoDto,
    #[serde(rename = "metadataId")]
    mosaic_id: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MosaicMetadataDto {
    metadata: MosaicMetadataInfoDto,
}

impl MosaicMetadataInfoDto {
    pub fn compact(&self) -> crate::Result<MosaicMetadataInfo> {
        let info = self.metadata.compact();

        let mosaic_id = if !self.mosaic_id.0.is_empty() {
            MosaicId::from(self.mosaic_id.compact())
        } else {
            MosaicId::default()
        };
        Ok(MosaicMetadataInfo { info, mosaic_id })
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NamespaceMetadataInfoDto {
    #[serde(flatten)]
    metadata: MetadataInfoDto,
    #[serde(rename = "metadataId")]
    namespace_id: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NamespaceMetadataDto {
    metadata: NamespaceMetadataInfoDto,
}

impl NamespaceMetadataInfoDto {
    pub fn compact(&self) -> crate::Result<NamespaceMetadataInfo> {
        let info = self.metadata.compact();

        let namespace_id = if !self.namespace_id.0.is_empty() {
            NamespaceId::from(self.namespace_id.compact())
        } else {
            NamespaceId::default()
        };
        Ok(NamespaceMetadataInfo { info, namespace_id })
    }
}
