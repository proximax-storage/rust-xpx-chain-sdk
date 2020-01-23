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
    pub start_height: Vec<i32>,
    #[serde(rename = "endHeight")]
    pub end_height: Vec<i32>,
    /// The level of the namespace.
    #[serde(rename = "depth")]
    pub depth: i32,
    #[serde(rename = "level0")]
    pub level0: Vec<i32>,
    #[serde(rename = "level1", skip_serializing_if = "Option::is_none")]
    pub level1: Option<Vec<i32>>,
    #[serde(rename = "level2", skip_serializing_if = "Option::is_none")]
    pub level2: Option<Vec<i32>>,
    #[serde(rename = "type")]
    pub _type: crate::models::NamespaceTypeEnum,
    #[serde(rename = "alias")]
    pub alias: crate::models::AliasDto,
    #[serde(rename = "parentId")]
    pub parent_id: Vec<i32>,
}

impl NamespaceDto {
    pub fn new(owner: String, owner_address: String, start_height: Vec<i32>, end_height: Vec<i32>, depth: i32, level0: Vec<i32>, _type: crate::models::NamespaceTypeEnum, alias: crate::models::AliasDto, parent_id: Vec<i32>) -> NamespaceDto {
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
    pub meta: crate::models::NamespaceMetaDto,
    #[serde(rename = "namespace")]
    pub namespace: crate::models::NamespaceDto,
}

impl NamespaceInfoDto {
    pub fn new(meta: crate::models::NamespaceMetaDto, namespace: crate::models::NamespaceDto) -> NamespaceInfoDto {
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
    pub metadata_id: Vec<i32>,
    #[serde(rename = "metadataType")]
    pub metadata_type: crate::models::MetadataTypeEnum,
    /// The array of metadata modifications.
    #[serde(rename = "modifications")]
    pub modifications: Vec<crate::models::MetadataModificationDto>,
}

impl NamespaceMetadataBodyDto {
    pub fn new(metadata_id: Vec<i32>, metadata_type: crate::models::MetadataTypeEnum, modifications: Vec<crate::models::MetadataModificationDto>) -> NamespaceMetadataBodyDto {
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
    pub fields: Vec<crate::models::FieldDto>,
    #[serde(rename = "metadataId")]
    pub metadata_id: Vec<i32>,
}

impl NamespaceMetadataDto {
    pub fn new(metadata_type: i32, fields: Vec<crate::models::FieldDto>, metadata_id: Vec<i32>) -> NamespaceMetadataDto {
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
    pub metadata_id: Vec<i32>,
}

impl NamespaceMetadataDtoAllOf {
    pub fn new(metadata_id: Vec<i32>) -> NamespaceMetadataDtoAllOf {
        NamespaceMetadataDtoAllOf {
            metadata_type: None,
            metadata_id,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceMetadataInfoDto {
    #[serde(rename = "metadata")]
    pub metadata: crate::models::NamespaceMetadataDto,
}

impl NamespaceMetadataInfoDto {
    pub fn new(metadata: crate::models::NamespaceMetadataDto) -> NamespaceMetadataInfoDto {
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
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - Public main network. * 0x98 (TEST_NET) - Public test network. * 0x60 (MIJIN) - Private network. * 0x90 (MIJIN_TEST) - Private test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
    #[serde(rename = "metadataId")]
    pub metadata_id: Vec<i32>,
    #[serde(rename = "metadataType")]
    pub metadata_type: crate::models::MetadataTypeEnum,
    /// The array of metadata modifications.
    #[serde(rename = "modifications")]
    pub modifications: Vec<crate::models::MetadataModificationDto>,
}

impl NamespaceMetadataTransactionDto {
    /// Transaction that addes metadata to namespace.
    pub fn new(signature: String, signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, metadata_id: Vec<i32>, metadata_type: crate::models::MetadataTypeEnum, modifications: Vec<crate::models::MetadataModificationDto>) -> NamespaceMetadataTransactionDto {
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
    pub namespace_id: Vec<i32>,
    /// The full name of the namespace.
    #[serde(rename = "name")]
    pub name: String,
}

impl NamespaceNameDto {
    pub fn new(namespace_id: Vec<i32>, name: String) -> NamespaceNameDto {
        NamespaceNameDto {
            namespace_id,
            name,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterNamespaceTransactionBodyDto {
    #[serde(rename = "namespaceType")]
    pub namespace_type: crate::models::NamespaceTypeEnum,
    #[serde(rename = "duration")]
    pub duration: Vec<i32>,
    #[serde(rename = "namespaceId")]
    pub namespace_id: Vec<i32>,
    /// The unique namespace name.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "parentId")]
    pub parent_id: Vec<i32>,
}

impl RegisterNamespaceTransactionBodyDto {
    pub fn new(namespace_type: crate::models::NamespaceTypeEnum, duration: Vec<i32>, namespace_id: Vec<i32>, name: String, parent_id: Vec<i32>) -> RegisterNamespaceTransactionBodyDto {
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
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - Public main network. * 0x98 (TEST_NET) - Public test network. * 0x60 (MIJIN) - Private network. * 0x90 (MIJIN_TEST) - Private test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
    #[serde(rename = "namespaceType")]
    pub namespace_type: crate::models::NamespaceTypeEnum,
    #[serde(rename = "duration")]
    pub duration: Vec<i32>,
    #[serde(rename = "namespaceId")]
    pub namespace_id: Vec<i32>,
    /// The unique namespace name.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "parentId")]
    pub parent_id: Vec<i32>,
}

impl RegisterNamespaceTransactionDto {
    /// Transaction that creates or renew a namespace.
    pub fn new(signature: String, signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, namespace_type: crate::models::NamespaceTypeEnum, duration: Vec<i32>, namespace_id: Vec<i32>, name: String, parent_id: Vec<i32>) -> RegisterNamespaceTransactionDto {
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
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - Public main network. * 0x98 (TEST_NET) - Public test network. * 0x60 (MIJIN) - Private network. * 0x90 (MIJIN_TEST) - Private test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
    #[serde(rename = "metadataId")]
    pub metadata_id: Vec<i32>,
    #[serde(rename = "metadataType")]
    pub metadata_type: crate::models::MetadataTypeEnum,
    /// The array of metadata modifications.
    #[serde(rename = "modifications")]
    pub modifications: Vec<crate::models::MetadataModificationDto>,
}

impl EmbeddedNamespaceMetadataTransactionDto {
    pub fn new(signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, metadata_id: Vec<i32>, metadata_type: crate::models::MetadataTypeEnum, modifications: Vec<crate::models::MetadataModificationDto>) -> EmbeddedNamespaceMetadataTransactionDto {
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
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - Public main network. * 0x98 (TEST_NET) - Public test network. * 0x60 (MIJIN) - Private network. * 0x90 (MIJIN_TEST) - Private test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
    #[serde(rename = "namespaceType")]
    pub namespace_type: crate::models::NamespaceTypeEnum,
    #[serde(rename = "duration")]
    pub duration: Vec<i32>,
    #[serde(rename = "namespaceId")]
    pub namespace_id: Vec<i32>,
    /// The unique namespace name.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "parentId")]
    pub parent_id: Vec<i32>,
}

impl EmbeddedRegisterNamespaceTransactionDto {
    pub fn new(signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, namespace_type: crate::models::NamespaceTypeEnum, duration: Vec<i32>, namespace_id: Vec<i32>, name: String, parent_id: Vec<i32>) -> EmbeddedRegisterNamespaceTransactionDto {
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

