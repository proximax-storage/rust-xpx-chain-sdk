use std::future::Future;

use hyper::client::connect::Connect;

use crate::apis::namespace_routes_api::NamespaceRoutes;
use crate::models::account::PublicAccount;
use crate::models::errors;
use crate::models::field_dto::FieldDto;
use crate::models::id_model::Id;
use crate::models::metadata_dto::{MetadataModificationDto, MetadataTypeEnum};
use crate::models::namespace::{NamespaceInfo, NamespaceName};
use crate::models::network::NetworkType;
use crate::models::transaction::{
    AbstractTransactionDto,
    RegisterNamespaceTransaction,
    Transaction, TransactionDto,
    TransactionMetaDto
};
use crate::models::uint_64::Uint64Dto;
use crate::Uint64;

use super::{NamespaceId, NamespaceType};

type NamespaceIdDto = Option<Uint64Dto>;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NamespaceDto {
    /// The public key of the owner of the namespace.
    pub owner: String,
    /// The address of the owner of the namespace in hexadecimal.
    pub owner_address: String,
    pub start_height: Uint64Dto,
    pub end_height: Uint64Dto,
    #[serde(rename = "depth")]
    pub depth: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level0: NamespaceIdDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level1: NamespaceIdDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level2: NamespaceIdDto,
    #[serde(rename = "type")]
    pub _type: u8,
    pub alias: crate::models::alias::AliasDto,
    pub parent_id: Uint64Dto,
}

impl NamespaceDto {
    fn extract_levels(&self) -> crate::Result<Vec<NamespaceId>> {
        let mut levels: Vec<NamespaceId> = Vec::new();

        let mut extract_level = |level: NamespaceIdDto| {
            if let Some(l) = level {
                let nemsapce_id = NamespaceId::from(l.to_struct());
                levels.push(nemsapce_id)
            }
        };

        extract_level(self.level0.to_owned());
        extract_level(self.level1.to_owned());
        extract_level(self.level2.to_owned());

        ensure!(
            !levels.is_empty(),
            errors::ERR_NULL_NAMESPACE_ID
         );

        Ok(levels)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NamespaceInfoDto {
    #[serde(rename = "meta")]
    pub meta: NamespaceMetaDto,
    #[serde(rename = "namespace")]
    pub namespace: NamespaceDto,
}

impl NamespaceInfoDto {
    pub fn to_struct(&self) -> crate::Result<NamespaceInfo>
    {
        let public_account = PublicAccount::from_public_key(
            &self.namespace.owner, NetworkType::from(self.namespace._type))?;

        let parent_id = NamespaceId::from(self.namespace.parent_id.to_struct());

        let levels = self.namespace.extract_levels()?;

        let alias = self.namespace.alias.to_struct()?;

        let mut namespace = NamespaceInfo {
            namespace_id: levels[levels.len() - 1],
            active: self.meta.active,
            type_space: NamespaceType::from(self.namespace._type),
            depth: self.namespace.depth,
            levels,
            alias,
            parent: None,
            owner: public_account,
            start_height: self.namespace.start_height.to_struct(),
            end_height: self.namespace.end_height.to_struct()
        };

        if parent_id.to_id() != Uint64::default() {
            let mut parent = NamespaceInfo {
                namespace_id: parent_id,
                active: false,
                type_space: NamespaceType::Root,
                depth: 1,
                levels: Default::default(),
                alias: Default::default(),
                parent: None,
                owner: Default::default(),
                start_height: Default::default(),
                end_height: Default::default()
            };
            namespace.parent = Some(Box::new(parent));
        }
        Ok(namespace)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NamespaceMetaDto {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "index")]
    pub index: i32,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NamespaceMetadataDto {
    #[serde(rename = "metadataType")]
    pub metadata_type: i32,
    #[serde(rename = "fields")]
    pub fields: Vec<FieldDto>,
    #[serde(rename = "metadataId")]
    pub metadata_id: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NamespaceMetadataDtoAllOf {
    #[serde(rename = "metadataType", skip_serializing_if = "Option::is_none")]
    pub metadata_type: Option<i32>,
    #[serde(rename = "metadataId")]
    pub metadata_id: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NamespaceMetadataInfoDto {
    #[serde(rename = "metadata")]
    pub metadata: NamespaceMetadataDto,
}

/// NamespaceMetadataTransactionDto : Transaction that addes metadata to namespace.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NamespaceMetadataTransactionDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    pub signer: String,
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<Uint64Dto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Uint64Dto>,
    pub metadata_id: Uint64Dto,
    pub metadata_type: MetadataTypeEnum,
    pub modifications: Vec<MetadataModificationDto>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NamespaceNameDto {
    #[serde(rename = "namespaceId")]
    pub namespace_id: Uint64Dto,
    /// The full name of the namespace.
    #[serde(rename = "name")]
    pub name: String,
}

impl NamespaceNameDto {
    pub fn to_struct(&self) -> crate::Result<NamespaceName> {
        Ok(NamespaceName{ namespace_id: NamespaceId::from(
            self.namespace_id.to_struct()),
            name: self.name.to_owned() })
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RegisterNamespaceTransactionInfoDto {
    pub meta: TransactionMetaDto,
    pub transaction: RegisterNamespaceTransactionDto,
}

/// RegisterNamespaceTransactionDto : Transaction that creates or renew a namespace.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RegisterNamespaceTransactionDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    pub signer: String,
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<Uint64Dto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Uint64Dto>,
    #[serde(rename = "namespaceType")]
    pub namespace_type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Uint64Dto>,
    pub namespace_id: Uint64Dto,
    #[serde(rename = "name")]
    pub namespace_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Uint64Dto>
}

#[typetag::serde]
impl TransactionDto for RegisterNamespaceTransactionInfoDto {
    fn to_struct(&self) -> crate::Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();

        let info = self.meta.to_struct();

        let abs_transaction = AbstractTransactionDto::new(
            dto.signature, dto.signer, dto.version, dto._type, dto.max_fee, dto.deadline,
        ).to_struct(info)?;

        let namespace_type = NamespaceType::from(dto.namespace_type);

        let namespace_id = NamespaceId::from(dto.namespace_id.to_struct());

        let mut parent_id = None;

        let mut duration = None;
        if namespace_type == NamespaceType::Root {
            if let Some(d) = dto.duration {
                duration = Some(d.to_struct())
            };
        } else {
            if let Some(p) = dto.parent_id {
                parent_id = Some(NamespaceId::from(p.to_struct()))
            };
        }

        Ok(Box::new(RegisterNamespaceTransaction {
            abs_transaction,
            namespace_type,
            namespace_id,
            name: dto.namespace_name,
            duration,
            parent_id
        }))
    }
}
