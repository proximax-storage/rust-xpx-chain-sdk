/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use downcast_rs::__std::collections::HashMap;

use crate::{
    api::metadata_dto_vec_to_struct,
    models::{
        account::Address,
        metadata::{
            AddressMetadataInfo, MetadataInfo, MetadataModification, MetadataModificationType,
            MetadataType, MosaicMetadataInfo, NamespaceMetadataInfo,
        },
        mosaic::MosaicId,
        namespace::NamespaceId,
        transaction::{ModifyMetadataTransaction, TransactionInfo},
    },
};

use super::{AbstractTransactionDto, FieldDto, Uint64Dto};

#[derive(Deserialize)]
pub(crate) struct MetadataDto<T> {
    pub(crate) metadata: T,
}

#[derive(Deserialize)]
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

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct MetadataModificationDto {
    #[serde(rename = "modificationType")]
    modification_type: u8,
    /// The key of metadata modification.
    key: String,
    /// The value of metadata modification.
    value: String,
}

impl MetadataModificationDto {
    pub fn compact(&self) -> MetadataModification {
        MetadataModification {
            r#type: MetadataModificationType::from(self.modification_type),
            key: self.key.clone(),
            value: self.value.clone(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ModifyMetadataTransactionDto {
    #[serde(flatten)]
    pub r#abstract: AbstractTransactionDto,
    pub metadata_type: u8,
    pub modifications: Vec<MetadataModificationDto>,
}

impl ModifyMetadataTransactionDto {
    pub fn compact(&self, info: TransactionInfo) -> crate::Result<ModifyMetadataTransaction> {
        let abs_transaction = self.r#abstract.compact(info)?;

        let modifications = metadata_dto_vec_to_struct(self.modifications.clone());
        Ok(ModifyMetadataTransaction {
            abs_transaction,
            metadata_type: MetadataType::from(self.metadata_type),
            modifications,
        })
    }
}

#[derive(Deserialize)]
pub(crate) struct AddressMetadataInfoDto {
    #[serde(flatten)]
    metadata: MetadataInfoDto,
    #[serde(rename = "metadataId")]
    address: String,
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

#[derive(Deserialize)]
pub(crate) struct MosaicMetadataInfoDto {
    #[serde(flatten)]
    metadata: MetadataInfoDto,
    #[serde(rename = "metadataId")]
    mosaic_id: Uint64Dto,
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

#[derive(Deserialize)]
pub(crate) struct NamespaceMetadataInfoDto {
    #[serde(flatten)]
    metadata: MetadataInfoDto,
    #[serde(rename = "metadataId")]
    namespace_id: Uint64Dto,
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
