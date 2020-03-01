use crate::models::field_dto::FieldDto;
use crate::models::metadata_dto::{MetadataModificationDto, MetadataTypeEnum};
use crate::models::uint_64::Uint64Dto;
use crate::models::transaction::{TransactionDto, Transaction, TransactionMetaDto, AbstractTransactionDto, RegisterNamespaceTransaction};
use failure::_core::any::Any;
use crate::models::namespace::{NamespaceId, NamespaceType};
use crate::models::Uint64;

use std::borrow::Cow;

#[derive(Serialize, Deserialize)]
pub struct NamespaceDto {
    /// The public key of the owner of the namespace.
    #[serde(rename = "owner")]
    pub owner: String,
    /// The address of the owner of the namespace in hexadecimal.
    #[serde(rename = "ownerAddress")]
    pub owner_address: String,
    #[serde(rename = "startHeight")]
    pub start_height: Uint64Dto,
    #[serde(rename = "endHeight")]
    pub end_height: Uint64Dto,
    /// The level of the namespace.
    #[serde(rename = "depth")]
    pub depth: i32,
    #[serde(rename = "level0")]
    pub level0: Uint64Dto,
    #[serde(rename = "level1", skip_serializing_if = "Option::is_none")]
    pub level1: Option<Uint64Dto>,
    #[serde(rename = "level2", skip_serializing_if = "Option::is_none")]
    pub level2: Option<Uint64Dto>,
    #[serde(rename = "type")]
    pub _type: u8,
    #[serde(rename = "alias")]
    pub alias: crate::models::alias::AliasDto,
    #[serde(rename = "parentId")]
    pub parent_id: Uint64Dto,
}

impl NamespaceDto {
    pub fn new(owner: String, owner_address: String, start_height: Uint64Dto, end_height: Uint64Dto, depth: i32, level0: Uint64Dto, _type: u8, alias: crate::models::alias::AliasDto, parent_id: Uint64Dto) -> NamespaceDto {
        NamespaceDto {
            owner,
            owner_address,
            start_height,
            end_height,
            depth,
            level0,
            level1: None,
            level2: None,
            _type,
            alias,
            parent_id,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NamespaceIds {
    /// The array of namespace identifiers.
    #[serde(rename = "namespaceIds", skip_serializing_if = "Option::is_none")]
    pub namespace_ids: Option<Vec<String>>,
}

impl NamespaceIds {
    pub fn new() -> NamespaceIds {
        NamespaceIds {
            namespace_ids: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NamespaceInfoDto {
    #[serde(rename = "meta")]
    pub meta: crate::models::namespace::NamespaceMetaDto,
    #[serde(rename = "namespace")]
    pub namespace: crate::models::namespace::NamespaceDto,
}

impl NamespaceInfoDto {
    pub fn new(meta: crate::models::namespace::NamespaceMetaDto, namespace: crate::models::namespace::NamespaceDto) -> NamespaceInfoDto {
        NamespaceInfoDto {
            meta,
            namespace,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NamespaceMetaDto {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "index")]
    pub index: i32,
}

impl NamespaceMetaDto {
    pub fn new(id: String, active: bool, index: i32) -> NamespaceMetaDto {
        NamespaceMetaDto {
            id,
            active,
            index,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NamespaceMetadataBodyDto {
    #[serde(rename = "metadataId")]
    pub metadata_id: Uint64Dto,
    #[serde(rename = "metadataType")]
    pub metadata_type: MetadataTypeEnum,
    /// The array of metadata modifications.
    #[serde(rename = "modifications")]
    pub modifications: Vec<MetadataModificationDto>,
}

impl NamespaceMetadataBodyDto {
    pub fn new(metadata_id: Uint64Dto, metadata_type: MetadataTypeEnum, modifications: Vec<MetadataModificationDto>) -> NamespaceMetadataBodyDto {
        NamespaceMetadataBodyDto {
            metadata_id,
            metadata_type,
            modifications,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NamespaceMetadataDto {
    #[serde(rename = "metadataType")]
    pub metadata_type: i32,
    #[serde(rename = "fields")]
    pub fields: Vec<FieldDto>,
    #[serde(rename = "metadataId")]
    pub metadata_id: Uint64Dto,
}

impl NamespaceMetadataDto {
    pub fn new(metadata_type: i32, fields: Vec<FieldDto>, metadata_id: Uint64Dto) -> NamespaceMetadataDto {
        NamespaceMetadataDto {
            metadata_type,
            fields,
            metadata_id,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NamespaceMetadataDtoAllOf {
    #[serde(rename = "metadataType", skip_serializing_if = "Option::is_none")]
    pub metadata_type: Option<i32>,
    #[serde(rename = "metadataId")]
    pub metadata_id: Uint64Dto,
}

impl NamespaceMetadataDtoAllOf {
    pub fn new(metadata_id: Uint64Dto) -> NamespaceMetadataDtoAllOf {
        NamespaceMetadataDtoAllOf {
            metadata_type: None,
            metadata_id,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NamespaceMetadataInfoDto {
    #[serde(rename = "metadata")]
    pub metadata: NamespaceMetadataDto,
}

impl NamespaceMetadataInfoDto {
    pub fn new(metadata: NamespaceMetadataDto) -> NamespaceMetadataInfoDto {
        NamespaceMetadataInfoDto {
            metadata,
        }
    }
}

/// NamespaceMetadataTransactionDto : Transaction that addes metadata to namespace.
#[derive(Serialize, Deserialize)]
pub struct NamespaceMetadataTransactionDto {
    /// The signature of the entity. The signature was generated by the signer and can be used to validate tha the entity data was not modified by a node.
    #[serde(rename = "signature")]
    pub signature: String,
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: u16,
    #[serde(rename = "max_fee")]
    pub max_fee: Uint64Dto,
    #[serde(rename = "deadline")]
    pub deadline: Uint64Dto,
    #[serde(rename = "metadataId")]
    pub metadata_id: Uint64Dto,
    #[serde(rename = "metadataType")]
    pub metadata_type: MetadataTypeEnum,
    /// The array of metadata modifications.
    #[serde(rename = "modifications")]
    pub modifications: Vec<MetadataModificationDto>,
}

#[derive(Serialize, Deserialize)]
pub struct NamespaceNameDto {
    #[serde(rename = "namespace_id")]
    pub namespace_id: Uint64Dto,
    /// The full name of the namespace.
    #[serde(rename = "name")]
    pub name: String,
}

impl NamespaceNameDto {
    pub fn new(namespace_id: Uint64Dto, name: String) -> NamespaceNameDto {
        NamespaceNameDto {
            namespace_id,
            name,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterNamespaceTransactionInfoDto {
    pub meta: TransactionMetaDto,
    pub transaction: RegisterNamespaceTransactionDto,
}

/// RegisterNamespaceTransactionDto : Transaction that creates or renew a namespace.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterNamespaceTransactionDto {
    pub signature: String,
    pub signer: String,
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: u16,
    pub max_fee: Uint64Dto,
    pub deadline: Uint64Dto,
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

        let abs_transaction = AbstractTransactionDto::new(
            dto.signature, dto.signer, dto.version, dto._type, dto.max_fee, dto.deadline,
        ).to_struct()?;

        let namespace_type = NamespaceType::from(dto.namespace_type);

        let namespace_id = NamespaceId::from(dto.namespace_id.to_struct());

        let mut parent_id = None ;

        let mut duration = None;
        if namespace_type  == NamespaceType::Root {
            if let Some(d) = dto.duration {
                duration = Some(d.to_struct())
            };
        } else {
            if let Some(p) = dto.parent_id {
                parent_id = Some(NamespaceId::from(p.to_struct()))
            };
        }

        Ok(Box::new(RegisterNamespaceTransaction{
            abs_transaction,
            namespace_type,
            namespace_id,
            name: dto.namespace_name,
            duration,
            parent_id
        }))
    }
}
