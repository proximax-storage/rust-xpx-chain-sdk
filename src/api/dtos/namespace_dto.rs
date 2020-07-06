/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::{
    account::PublicAccount,
    errors_const,
    models::Result,
    namespace::{NamespaceId, NamespaceInfo, NamespaceName, NamespaceType},
    network::NetworkType,
    transaction::{RegisterNamespaceTransaction, Transaction},
    AssetId,
};

use super::{
    AbstractTransactionDto, AliasDto, FieldDto, MetadataModificationDto, MetadataTypeEnum,
    TransactionDto, TransactionMetaDto, Uint64Dto,
};

type NamespaceIdDto = Option<Uint64Dto>;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NamespaceDto {
    /// The public key of the owner of the namespace.
    owner: String,
    /// The address of the owner of the namespace in hexadecimal.
    owner_address: String,
    start_height: Uint64Dto,
    end_height: Uint64Dto,
    #[serde(rename = "depth")]
    depth: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    level0: NamespaceIdDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    level1: NamespaceIdDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    level2: NamespaceIdDto,
    #[serde(rename = "type")]
    _type: u8,
    alias: AliasDto,
    parent_id: Uint64Dto,
}

impl NamespaceDto {
    fn extract_levels(&self) -> crate::Result<Vec<NamespaceId>> {
        let mut levels: Vec<NamespaceId> = Vec::new();

        let mut extract_level = |level: NamespaceIdDto| {
            if let Some(l) = level {
                let nemsapce_id = NamespaceId::from(l.compact());
                levels.push(nemsapce_id)
            }
        };

        extract_level(self.level0.to_owned());
        extract_level(self.level1.to_owned());
        extract_level(self.level2.to_owned());

        ensure!(!levels.is_empty(), errors_const::ERR_EMPTY_NAMESPACE_ID);

        Ok(levels)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NamespaceInfoDto {
    #[serde(rename = "meta")]
    meta: NamespaceMetaDto,
    #[serde(rename = "namespace")]
    namespace: NamespaceDto,
}

impl NamespaceInfoDto {
    pub fn compact(&self) -> crate::Result<NamespaceInfo> {
        let public_account = PublicAccount::from_public_key(
            &self.namespace.owner,
            NetworkType::from(NetworkType::from(self.namespace._type)),
        )?;

        let levels = self.namespace.extract_levels()?;

        let parent_id = NamespaceId::from(self.namespace.parent_id.compact());

        let alias = self.namespace.alias.compact()?;

        let mut namespace = NamespaceInfo {
            namespace_id: levels[levels.len() - 1],
            active: self.meta.active,
            type_space: NamespaceType::from(self.namespace._type),
            depth: self.namespace.depth,
            levels,
            alias,
            parent: None,
            owner: public_account,
            start_height: self.namespace.start_height.compact(),
            end_height: self.namespace.end_height.compact(),
        };

        if parent_id.to_u64() != 0 {
            let parent = NamespaceInfo {
                namespace_id: parent_id,
                active: false,
                type_space: NamespaceType::Root,
                depth: 1,
                levels: Default::default(),
                alias: Default::default(),
                parent: None,
                owner: Default::default(),
                start_height: Default::default(),
                end_height: Default::default(),
            };
            namespace.parent = Some(Box::new(parent));
        }

        Ok(namespace)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NamespaceMetaDto {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "active")]
    active: bool,
    #[serde(rename = "index")]
    index: i32,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NamespaceMetadataDto {
    #[serde(rename = "metadataType")]
    metadata_type: i32,
    #[serde(rename = "fields")]
    fields: Vec<FieldDto>,
    #[serde(rename = "metadataId")]
    metadata_id: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NamespaceMetadataDtoAllOf {
    #[serde(rename = "metadataType", skip_serializing_if = "Option::is_none")]
    metadata_type: Option<i32>,
    #[serde(rename = "metadataId")]
    metadata_id: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NamespaceMetadataInfoDto {
    #[serde(rename = "metadata")]
    metadata: NamespaceMetadataDto,
}

/// NamespaceMetadataTransactionDto : Transaction that addes metadata to namespace.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NamespaceMetadataTransactionDto {
    #[serde(flatten)]
    r#abstract: AbstractTransactionDto,
    metadata_id: Uint64Dto,
    metadata_type: MetadataTypeEnum,
    modifications: Vec<MetadataModificationDto>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NamespaceNameDto {
    #[serde(rename = "namespaceId")]
    namespace_id: Uint64Dto,
    /// The full name of the namespace.
    #[serde(rename = "name")]
    name: String,
}

impl NamespaceNameDto {
    pub fn compact(&self) -> crate::Result<NamespaceName> {
        Ok(NamespaceName {
            namespace_id: NamespaceId::from(self.namespace_id.compact()),
            name: self.name.to_owned(),
        })
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RegisterNamespaceTransactionInfoDto {
    meta: TransactionMetaDto,
    transaction: RegisterNamespaceTransactionDto,
}

/// RegisterNamespaceTransactionDto : Transaction that creates or renew a namespace.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RegisterNamespaceTransactionDto {
    #[serde(flatten)]
    r#abstract: AbstractTransactionDto,
    #[serde(rename = "namespaceType")]
    namespace_type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Uint64Dto>,
    namespace_id: Uint64Dto,
    #[serde(rename = "name")]
    namespace_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<Uint64Dto>,
}

#[typetag::serde]
impl TransactionDto for RegisterNamespaceTransactionInfoDto {
    fn compact(&self) -> Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();

        let info = self.meta.compact();

        let abs_transaction = dto.r#abstract.compact(info)?;

        let namespace_type = NamespaceType::from(dto.namespace_type);

        let namespace_id = NamespaceId::from(dto.namespace_id.compact());

        let mut parent_id = None;

        let mut duration = None;
        if namespace_type == NamespaceType::Root {
            if let Some(d) = dto.duration {
                duration = Some(d.compact())
            };
        } else {
            if let Some(p) = dto.parent_id {
                parent_id = Some(NamespaceId::from(p.compact()))
            };
        }

        Ok(Box::new(RegisterNamespaceTransaction {
            abs_transaction,
            namespace_type,
            namespace_id,
            name: dto.namespace_name,
            duration,
            parent_id,
        }))
    }
}
