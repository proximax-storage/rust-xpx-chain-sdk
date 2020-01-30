/// MosaicDirectionEnum : The supply modification direction: * 0  - Decrease. * 1  - Increase.
/// The supply modification direction: * 0  - Decrease. * 1  - Increase.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MosaicDirectionEnum {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,
}

/// MosaicPropertyIdEnum : The mosaic propery id means: * 0 - MosaicFlags * 1 - divisibility * 2 - Duration
/// The mosaic propery id means: * 0 - MosaicFlags * 1 - divisibility * 2 - Duration
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MosaicPropertyIdEnum {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,

}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicDto {
    #[serde(rename = "id")]
    pub id: Vec<i32>,
    #[serde(rename = "amount")]
    pub amount: Vec<i32>,
}

impl MosaicDto {
    pub fn new(id: Vec<i32>, amount: Vec<i32>) -> MosaicDto {
        MosaicDto {
            id,
            amount,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicIds {
    /// The array of mosaic identifiers.
    #[serde(rename = "mosaicIds", skip_serializing_if = "Option::is_none")]
    pub mosaic_ids: Option<Vec<String>>,
}

impl MosaicIds {
    pub fn new() -> MosaicIds {
        MosaicIds {
            mosaic_ids: None,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicInfoDto {
    #[serde(rename = "meta")]
    pub meta: crate::models::mosaic::MosaicMetaDto,
    #[serde(rename = "mosaic")]
    pub mosaic: crate::models::mosaic::MosaicDefinitionDto,
}

impl MosaicInfoDto {
    pub fn new(meta: crate::models::mosaic::MosaicMetaDto, mosaic: crate::models::mosaic::MosaicDefinitionDto) -> MosaicInfoDto {
        MosaicInfoDto {
            meta,
            mosaic,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicMetaDto {
    #[serde(rename = "id")]
    pub id: String,
}

impl MosaicMetaDto {
    pub fn new(id: String) -> MosaicMetaDto {
        MosaicMetaDto {
            id,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicMetadataBodyDto {
    #[serde(rename = "metadataId")]
    pub metadata_id: Vec<i32>,
    #[serde(rename = "metadataType")]
    pub metadata_type: crate::models::MetadataTypeEnum,
    /// The array of metadata modifications.
    #[serde(rename = "modifications")]
    pub modifications: Vec<crate::models::MetadataModificationDto>,
}

impl MosaicMetadataBodyDto {
    pub fn new(metadata_id: Vec<i32>, metadata_type: crate::models::MetadataTypeEnum, modifications: Vec<crate::models::MetadataModificationDto>) -> MosaicMetadataBodyDto {
        MosaicMetadataBodyDto {
            metadata_id,
            metadata_type,
            modifications,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicDefinitionDto {
    #[serde(rename = "mosaic_id")]
    pub mosaic_id: Vec<i32>,
    #[serde(rename = "supply")]
    pub supply: Vec<i32>,
    #[serde(rename = "height")]
    pub height: Vec<i32>,
    /// The public key of the mosaic owner.
    #[serde(rename = "owner")]
    pub owner: String,
    /// The number of definitions for the same mosaic.
    #[serde(rename = "revision")]
    pub revision: i32,
    #[serde(rename = "properties")]
    pub properties: Vec<crate::models::mosaic::MosaicPropertyDto>,
}

impl MosaicDefinitionDto {
    pub fn new(mosaic_id: Vec<i32>, supply: Vec<i32>, height: Vec<i32>, owner: String, revision: i32, properties: Vec<crate::models::mosaic::MosaicPropertyDto>) -> MosaicDefinitionDto {
        MosaicDefinitionDto {
            mosaic_id,
            supply,
            height,
            owner,
            revision,
            properties,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicMetadataDto {
    #[serde(rename = "metadataType")]
    pub metadata_type: i32,
    #[serde(rename = "fields")]
    pub fields: Vec<crate::models::FieldDto>,
    #[serde(rename = "metadataId")]
    pub metadata_id: Vec<i32>,
}

impl MosaicMetadataDto {
    pub fn new(metadata_type: i32, fields: Vec<crate::models::FieldDto>, metadata_id: Vec<i32>) -> MosaicMetadataDto {
        MosaicMetadataDto {
            metadata_type,
            fields,
            metadata_id,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicMetadataDtoAllOf {
    #[serde(rename = "metadataType", skip_serializing_if = "Option::is_none")]
    pub metadata_type: Option<i32>,
    #[serde(rename = "metadataId")]
    pub metadata_id: Vec<i32>,
}

impl MosaicMetadataDtoAllOf {
    pub fn new(metadata_id: Vec<i32>) -> MosaicMetadataDtoAllOf {
        MosaicMetadataDtoAllOf {
            metadata_type: None,
            metadata_id,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicMetadataInfoDto {
    #[serde(rename = "metadata")]
    pub metadata: crate::models::mosaic::MosaicMetadataDto,
}

impl MosaicMetadataInfoDto {
    pub fn new(metadata: crate::models::mosaic::MosaicMetadataDto) -> MosaicMetadataInfoDto {
        MosaicMetadataInfoDto {
            metadata,
        }
    }
}

/// MosaicMetadataTransactionDto : Transaction that addes metadata to mosaic.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicMetadataTransactionDto {
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

impl MosaicMetadataTransactionDto {
    /// Transaction that addes metadata to mosaic.
    pub fn new(signature: String, signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, metadata_id: Vec<i32>, metadata_type: crate::models::MetadataTypeEnum, modifications: Vec<crate::models::MetadataModificationDto>) -> MosaicMetadataTransactionDto {
        MosaicMetadataTransactionDto {
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
pub struct MosaicNamesDto {
    #[serde(rename = "mosaic_id")]
    pub mosaic_id: Vec<i32>,
    /// The mosaic linked namespace names.
    #[serde(rename = "names")]
    pub names: Vec<String>,
}

impl MosaicNamesDto {
    pub fn new(mosaic_id: Vec<i32>, names: Vec<String>) -> MosaicNamesDto {
        MosaicNamesDto {
            mosaic_id,
            names,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicPropertyDto {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::models::mosaic::MosaicPropertyIdEnum>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<i32>>,
}

impl MosaicPropertyDto {
    pub fn new() -> MosaicPropertyDto {
        MosaicPropertyDto {
            id: None,
            value: None,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicSupplyChangeTransactionBodyDto {
    #[serde(rename = "mosaic_id")]
    pub mosaic_id: Vec<i32>,
    #[serde(rename = "direction")]
    pub direction: crate::models::mosaic::MosaicDirectionEnum,
    #[serde(rename = "delta")]
    pub delta: Vec<i32>,
}

impl MosaicSupplyChangeTransactionBodyDto {
    pub fn new(mosaic_id: Vec<i32>, direction: crate::models::mosaic::MosaicDirectionEnum, delta: Vec<i32>) -> MosaicSupplyChangeTransactionBodyDto {
        MosaicSupplyChangeTransactionBodyDto {
            mosaic_id,
            direction,
            delta,
        }
    }
}

/// MosaicSupplyChangeTransactionDto : Transaction to increase or decrease a mosaic’s supply.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicSupplyChangeTransactionDto {
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
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
    #[serde(rename = "mosaic_id")]
    pub mosaic_id: Vec<i32>,
    #[serde(rename = "direction")]
    pub direction: crate::models::mosaic::MosaicDirectionEnum,
    #[serde(rename = "delta")]
    pub delta: Vec<i32>,
}

impl MosaicSupplyChangeTransactionDto {
    /// Transaction to increase or decrease a mosaic’s supply.
    pub fn new(signature: String, signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, mosaic_id: Vec<i32>, direction: crate::models::mosaic::MosaicDirectionEnum, delta: Vec<i32>) -> MosaicSupplyChangeTransactionDto {
        MosaicSupplyChangeTransactionDto {
            signature,
            signer,
            version,
            _type,
            max_fee,
            deadline,
            mosaic_id,
            direction,
            delta,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicDefinitionTransactionBodyDto {
    /// Random nonce used to generate the mosaic id.
    #[serde(rename = "mosaicNonce")]
    pub mosaic_nonce: i32,
    #[serde(rename = "mosaic_id")]
    pub mosaic_id: Vec<i32>,
    #[serde(rename = "properties")]
    pub properties: Vec<crate::models::mosaic::MosaicPropertyDto>,
}

impl MosaicDefinitionTransactionBodyDto {
    pub fn new(mosaic_nonce: i32, mosaic_id: Vec<i32>, properties: Vec<crate::models::mosaic::MosaicPropertyDto>) -> MosaicDefinitionTransactionBodyDto {
        MosaicDefinitionTransactionBodyDto {
            mosaic_nonce,
            mosaic_id,
            properties,
        }
    }
}

/// MosaicDefinitionTransactionDto : Transaction that creates a new mosaic.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicDefinitionTransactionDto {
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
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
    /// Random nonce used to generate the mosaic id.
    #[serde(rename = "mosaicNonce")]
    pub mosaic_nonce: i32,
    #[serde(rename = "mosaic_id")]
    pub mosaic_id: Vec<i32>,
    #[serde(rename = "properties")]
    pub properties: Vec<crate::models::mosaic::MosaicPropertyDto>,
}

impl MosaicDefinitionTransactionDto {
    /// Transaction that creates a new mosaic.
    pub fn new(signature: String, signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, mosaic_nonce: i32, mosaic_id: Vec<i32>, properties: Vec<crate::models::mosaic::MosaicPropertyDto>) -> MosaicDefinitionTransactionDto {
        MosaicDefinitionTransactionDto {
            signature,
            signer,
            version,
            _type,
            max_fee,
            deadline,
            mosaic_nonce,
            mosaic_id,
            properties,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedMosaicDefinitionTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
    /// Random nonce used to generate the mosaic id.
    #[serde(rename = "mosaicNonce")]
    pub mosaic_nonce: i32,
    #[serde(rename = "mosaic_id")]
    pub mosaic_id: Vec<i32>,
    #[serde(rename = "properties")]
    pub properties: Vec<crate::models::mosaic::MosaicPropertyDto>,
}

impl EmbeddedMosaicDefinitionTransactionDto {
    pub fn new(signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, mosaic_nonce: i32, mosaic_id: Vec<i32>, properties: Vec<crate::models::mosaic::MosaicPropertyDto>) -> EmbeddedMosaicDefinitionTransactionDto {
        EmbeddedMosaicDefinitionTransactionDto {
            signer,
            version,
            _type,
            max_fee,
            deadline,
            mosaic_nonce,
            mosaic_id,
            properties,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedMosaicMetadataTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
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

impl EmbeddedMosaicMetadataTransactionDto {
    pub fn new(signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, metadata_id: Vec<i32>, metadata_type: crate::models::MetadataTypeEnum, modifications: Vec<crate::models::MetadataModificationDto>) -> EmbeddedMosaicMetadataTransactionDto {
        EmbeddedMosaicMetadataTransactionDto {
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
pub struct EmbeddedMosaicSupplyChangeTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
    #[serde(rename = "mosaic_id")]
    pub mosaic_id: Vec<i32>,
    #[serde(rename = "direction")]
    pub direction: crate::models::mosaic::MosaicDirectionEnum,
    #[serde(rename = "delta")]
    pub delta: Vec<i32>,
}

impl EmbeddedMosaicSupplyChangeTransactionDto {
    pub fn new(signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, mosaic_id: Vec<i32>, direction: crate::models::mosaic::MosaicDirectionEnum, delta: Vec<i32>) -> EmbeddedMosaicSupplyChangeTransactionDto {
        EmbeddedMosaicSupplyChangeTransactionDto {
            signer,
            version,
            _type,
            max_fee,
            deadline,
            mosaic_id,
            direction,
            delta,
        }
    }
}
