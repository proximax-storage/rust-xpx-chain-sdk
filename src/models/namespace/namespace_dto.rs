use crate::models::metadata_dto::{MetadataTypeEnum, MetadataModificationDto};
use crate::models::field_dto::FieldDto;
use crate::models::entity_dto::EntityType;
use crate::models::uint_64::Uint64Dto;

/// NamespaceTypeEnum : The namespace type: * 0 -  Root namespace. * 1 -  Subnamespace.
/// The namespace type: * 0 -  Root namespace. * 1 -  Subnamespace.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum NamespaceTypeEnum {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
    pub _type: crate::models::namespace::NamespaceTypeEnum,
    #[serde(rename = "alias")]
    pub alias: crate::models::alias::AliasDto,
    #[serde(rename = "parentId")]
    pub parent_id: Uint64Dto,
}

impl NamespaceDto {
    pub fn new(owner: String, owner_address: String, start_height: Uint64Dto, end_height: Uint64Dto, depth: i32, level0: Uint64Dto, _type: crate::models::namespace::NamespaceTypeEnum, alias: crate::models::alias::AliasDto, parent_id: Uint64Dto) -> NamespaceDto {
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
    pub _type: EntityType,
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

impl NamespaceMetadataTransactionDto {
    /// Transaction that addes metadata to namespace.
    pub fn new(signature: String, signer: String, version: i32, _type: EntityType, max_fee: Uint64Dto, deadline: Uint64Dto, metadata_id: Uint64Dto, metadata_type: MetadataTypeEnum, modifications: Vec<MetadataModificationDto>) -> NamespaceMetadataTransactionDto {
        NamespaceMetadataTransactionDto {
            signature,
            signer,
            version,
            _type,
            max_fee,
            deadline,
            metadata_id,
            metadata_type,
            modifications,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceNameDto {
    #[serde(rename = "namespaceId")]
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterNamespaceTransactionBodyDto {
    #[serde(rename = "namespaceType")]
    pub namespace_type: NamespaceTypeEnum,
    #[serde(rename = "duration")]
    pub duration: Uint64Dto,
    #[serde(rename = "namespaceId")]
    pub namespace_id: Uint64Dto,
    /// The unique namespace name.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "parentId")]
    pub parent_id: Uint64Dto,
}

impl RegisterNamespaceTransactionBodyDto {
    pub fn new(namespace_type: NamespaceTypeEnum, duration: Uint64Dto, namespace_id: Uint64Dto, name: String, parent_id: Uint64Dto) -> RegisterNamespaceTransactionBodyDto {
        RegisterNamespaceTransactionBodyDto {
            namespace_type,
            duration,
            namespace_id,
            name,
            parent_id,
        }
    }
}

/// RegisterNamespaceTransactionDto : Transaction that creates or renew a namespace.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterNamespaceTransactionDto {
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
    pub _type: EntityType,
    #[serde(rename = "max_fee")]
    pub max_fee: Uint64Dto,
    #[serde(rename = "deadline")]
    pub deadline: Uint64Dto,
    #[serde(rename = "namespaceType")]
    pub namespace_type: NamespaceTypeEnum,
    #[serde(rename = "duration")]
    pub duration: Uint64Dto,
    #[serde(rename = "namespaceId")]
    pub namespace_id: Uint64Dto,
    /// The unique namespace name.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "parentId")]
    pub parent_id: Uint64Dto,
}

impl RegisterNamespaceTransactionDto {
    /// Transaction that creates or renew a namespace.
    pub fn new(signature: String, signer: String, version: i32, _type: EntityType, max_fee: Uint64Dto, deadline: Uint64Dto, namespace_type: crate::models::namespace::NamespaceTypeEnum, duration: Uint64Dto, namespace_id: Uint64Dto, name: String, parent_id: Uint64Dto) -> RegisterNamespaceTransactionDto {
        RegisterNamespaceTransactionDto {
            signature,
            signer,
            version,
            _type,
            max_fee,
            deadline,
            namespace_type,
            duration,
            namespace_id,
            name,
            parent_id,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedNamespaceMetadataTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: EntityType,
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

impl EmbeddedNamespaceMetadataTransactionDto {
    pub fn new(signer: String, version: i32, _type: EntityType, max_fee: Uint64Dto, deadline: Uint64Dto, metadata_id: Uint64Dto, metadata_type: MetadataTypeEnum, modifications: Vec<MetadataModificationDto>) -> EmbeddedNamespaceMetadataTransactionDto {
        EmbeddedNamespaceMetadataTransactionDto {
            signer,
            version,
            _type,
            max_fee,
            deadline,
            metadata_id,
            metadata_type,
            modifications,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedRegisterNamespaceTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: EntityType,
    #[serde(rename = "max_fee")]
    pub max_fee: Uint64Dto,
    #[serde(rename = "deadline")]
    pub deadline: Uint64Dto,
    #[serde(rename = "namespaceType")]
    pub namespace_type: NamespaceTypeEnum,
    #[serde(rename = "duration")]
    pub duration: Uint64Dto,
    #[serde(rename = "namespaceId")]
    pub namespace_id: Uint64Dto,
    /// The unique namespace name.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "parentId")]
    pub parent_id: Uint64Dto,
}

impl EmbeddedRegisterNamespaceTransactionDto {
    pub fn new(signer: String, version: i32, _type: EntityType, max_fee: Uint64Dto, deadline: Uint64Dto, namespace_type: crate::models::namespace::NamespaceTypeEnum, duration: Uint64Dto, namespace_id: Uint64Dto, name: String, parent_id: Uint64Dto) -> EmbeddedRegisterNamespaceTransactionDto {
        EmbeddedRegisterNamespaceTransactionDto {
            signer,
            version,
            _type,
            max_fee,
            deadline,
            namespace_type,
            duration,
            namespace_id,
            name,
            parent_id,
        }
    }
}

