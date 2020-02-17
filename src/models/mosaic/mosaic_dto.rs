use crate::models::{MetadataModificationDto, Uint64Dto, Uint64};
use crate::models::mosaic::{Mosaic, MosaicId, MosaicInfo, MosaicProperties, SUPPLY_MUTABLE, TRANSFERABLE};
use crate::Result;
use crate::models::mosaic::mosaic_internal::{MosaicPropertyId, has_bits};
use chrono::format::Item::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MosaicDto {
    #[serde(rename = "id")]
    pub id: Uint64Dto,
    #[serde(rename = "amount")]
    pub amount: Uint64Dto,
}

impl MosaicDto {
    pub fn to_struct(&self) -> Mosaic {
        let mosaic_id = Box::new(MosaicId::from(self.id.to_struct()));
        let amount = self.amount.to_struct();
        Mosaic::new(mosaic_id, amount)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
pub(crate) struct MosaicInfoDto {
    #[serde(rename = "meta")]
    meta: MosaicMetaDto,
    #[serde(rename = "mosaic")]
    mosaic: MosaicDefinitionDto,
}

impl MosaicInfoDto {
    pub fn to_struct(&self) -> Result<MosaicInfo> {
        ensure!(
            self.mosaic.properties.len() > 0,
            "mosaic Properties is not valid."
         );

        let mosaic_id = MosaicId::from(self.mosaic.mosaic_id.to_struct());

        let properties =  Self::mosaic_properties(&self.mosaic.properties)?;

        Ok(MosaicInfo::new(
            mosaic_id,
            self.mosaic.supply.to_struct(),
            self.mosaic.height.to_struct(),
            (&self.mosaic.owner).parse()?,
            self.mosaic.revision,
            properties,
        ))
    }

    fn mosaic_properties(dto: &Vec<MosaicPropertyDto>) -> Result<MosaicProperties> {
        let mut flags: Uint64  = Uint64::default();
        let mut divisibility: u8 = 0;
        let mut duration: Uint64 = Uint64::default();


        for property  in dto {
            match property.id {
                0 => flags = property.value.to_struct(),
                1 => divisibility = property.value.to_struct().0 as u8,
                2 => duration = property.value.to_struct(),
                _ => bail!("Unknown Property Id")
            }
        };

        MosaicProperties::new(
            has_bits(flags, SUPPLY_MUTABLE),
            has_bits(flags, TRANSFERABLE),
            divisibility,
            duration,
        )
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MosaicMetaDto {
    #[serde(rename = "id")]
    id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicMetadataBodyDto {
    #[serde(rename = "metadataId")]
    pub metadata_id: Uint64Dto,
    #[serde(rename = "metadataType")]
    pub metadata_type: u16,
    /// The array of metadata modifications.
    #[serde(rename = "modifications")]
    pub modifications: Vec<MetadataModificationDto>,
}

impl MosaicMetadataBodyDto {
    pub fn new(metadata_id: Uint64Dto, metadata_type: u16, modifications: Vec<MetadataModificationDto>) -> MosaicMetadataBodyDto {
        MosaicMetadataBodyDto {
            metadata_id,
            metadata_type,
            modifications,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicDefinitionDto {
    mosaic_id: Uint64Dto,
    supply: Uint64Dto,
    height: Uint64Dto,
    owner: String,
    revision: usize,
    properties: Vec<MosaicPropertyDto>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicMetadataDto {
    #[serde(rename = "metadataType")]
    pub metadata_type: i32,
    #[serde(rename = "fields")]
    pub fields: Vec<crate::models::FieldDto>,
    #[serde(rename = "metadataId")]
    pub metadata_id: Uint64Dto,
}

impl MosaicMetadataDto {
    pub fn new(metadata_type: i32, fields: Vec<crate::models::FieldDto>, metadata_id: Uint64Dto) -> MosaicMetadataDto {
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
    pub metadata_id: Uint64Dto,
}

impl MosaicMetadataDtoAllOf {
    pub fn new(metadata_id: Uint64Dto) -> MosaicMetadataDtoAllOf {
        MosaicMetadataDtoAllOf {
            metadata_type: None,
            metadata_id,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicMetadataInfoDto {
    #[serde(rename = "metadata")]
    pub metadata: MosaicMetadataDto,
}

impl MosaicMetadataInfoDto {
    pub fn new(metadata: MosaicMetadataDto) -> MosaicMetadataInfoDto {
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
    pub max_fee: Uint64Dto,
    #[serde(rename = "deadline")]
    pub deadline: Uint64Dto,
    #[serde(rename = "metadataId")]
    pub metadata_id: Uint64Dto,
    #[serde(rename = "metadataType")]
    pub metadata_type: crate::models::MetadataTypeEnum,
    /// The array of metadata modifications.
    #[serde(rename = "modifications")]
    pub modifications: Vec<crate::models::MetadataModificationDto>,
}

impl MosaicMetadataTransactionDto {
    /// Transaction that addes metadata to mosaic.
    pub fn new(signature: String, signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Uint64Dto, deadline: Uint64Dto, metadata_id: Uint64Dto, metadata_type: crate::models::MetadataTypeEnum, modifications: Vec<crate::models::MetadataModificationDto>) -> MosaicMetadataTransactionDto {
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
    pub mosaic_id: Uint64Dto,
    /// The mosaic linked namespace names.
    #[serde(rename = "names")]
    pub names: Vec<String>,
}

impl MosaicNamesDto {
    pub fn new(mosaic_id: Uint64Dto, names: Vec<String>) -> MosaicNamesDto {
        MosaicNamesDto {
            mosaic_id,
            names,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicPropertyDto {
    pub id: u8,
    pub value: Uint64Dto,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicSupplyChangeTransactionBodyDto {
    #[serde(rename = "mosaic_id")]
    pub mosaic_id: Uint64Dto,
    #[serde(rename = "direction")]
    pub direction: u8,
    #[serde(rename = "delta")]
    pub delta: Uint64Dto,
}

impl MosaicSupplyChangeTransactionBodyDto {
    pub fn new(mosaic_id: Uint64Dto, direction: u8, delta: Uint64Dto) -> MosaicSupplyChangeTransactionBodyDto {
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
    pub _type: u8,
    #[serde(rename = "max_fee")]
    pub max_fee: Uint64Dto,
    #[serde(rename = "deadline")]
    pub deadline: Uint64Dto,
    #[serde(rename = "mosaic_id")]
    pub mosaic_id: Uint64Dto,
    #[serde(rename = "direction")]
    pub direction: u8,
    #[serde(rename = "delta")]
    pub delta: Uint64Dto,
}

impl MosaicSupplyChangeTransactionDto {
    /// Transaction to increase or decrease a mosaic’s supply.
    pub fn new(signature: String, signer: String, version: i32, _type: u8, max_fee: Uint64Dto, deadline: Uint64Dto, mosaic_id: Uint64Dto, direction: u8, delta: Uint64Dto) -> MosaicSupplyChangeTransactionDto {
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
    pub mosaic_id: Uint64Dto,
    #[serde(rename = "properties")]
    pub properties: Vec<MosaicPropertyDto>,
}

impl MosaicDefinitionTransactionBodyDto {
    pub fn new(mosaic_nonce: i32, mosaic_id: Uint64Dto, properties: Vec<crate::models::mosaic::MosaicPropertyDto>) -> MosaicDefinitionTransactionBodyDto {
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
    pub max_fee: Uint64Dto,
    #[serde(rename = "deadline")]
    pub deadline: Uint64Dto,
    /// Random nonce used to generate the mosaic id.
    #[serde(rename = "mosaicNonce")]
    pub mosaic_nonce: i32,
    #[serde(rename = "mosaic_id")]
    pub mosaic_id: Uint64Dto,
    #[serde(rename = "properties")]
    pub properties: Vec<crate::models::mosaic::MosaicPropertyDto>,
}

impl MosaicDefinitionTransactionDto {
    /// Transaction that creates a new mosaic.
    pub fn new(signature: String, signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Uint64Dto, deadline: Uint64Dto, mosaic_nonce: i32, mosaic_id: Uint64Dto, properties: Vec<crate::models::mosaic::MosaicPropertyDto>) -> MosaicDefinitionTransactionDto {
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
    pub max_fee: Uint64Dto,
    #[serde(rename = "deadline")]
    pub deadline: Uint64Dto,
    /// Random nonce used to generate the mosaic id.
    #[serde(rename = "mosaicNonce")]
    pub mosaic_nonce: i32,
    #[serde(rename = "mosaic_id")]
    pub mosaic_id: Uint64Dto,
    #[serde(rename = "properties")]
    pub properties: Vec<crate::models::mosaic::MosaicPropertyDto>,
}

impl EmbeddedMosaicDefinitionTransactionDto {
    pub fn new(signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Uint64Dto, deadline: Uint64Dto, mosaic_nonce: i32, mosaic_id: Uint64Dto, properties: Vec<crate::models::mosaic::MosaicPropertyDto>) -> EmbeddedMosaicDefinitionTransactionDto {
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
    pub max_fee: Uint64Dto,
    #[serde(rename = "deadline")]
    pub deadline: Uint64Dto,
    #[serde(rename = "metadataId")]
    pub metadata_id: Uint64Dto,
    #[serde(rename = "metadataType")]
    pub metadata_type: crate::models::MetadataTypeEnum,
    /// The array of metadata modifications.
    #[serde(rename = "modifications")]
    pub modifications: Vec<crate::models::MetadataModificationDto>,
}

impl EmbeddedMosaicMetadataTransactionDto {
    pub fn new(signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Uint64Dto, deadline: Uint64Dto, metadata_id: Uint64Dto, metadata_type: crate::models::MetadataTypeEnum, modifications: Vec<crate::models::MetadataModificationDto>) -> EmbeddedMosaicMetadataTransactionDto {
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
    pub _type: u8,
    #[serde(rename = "max_fee")]
    pub max_fee: Uint64Dto,
    #[serde(rename = "deadline")]
    pub deadline: Uint64Dto,
    #[serde(rename = "mosaic_id")]
    pub mosaic_id: Uint64Dto,
    #[serde(rename = "direction")]
    pub direction: u8,
    #[serde(rename = "delta")]
    pub delta: Uint64Dto,
}

impl EmbeddedMosaicSupplyChangeTransactionDto {
    pub fn new(signer: String, version: i32, _type: u8, max_fee: Uint64Dto, deadline: Uint64Dto, mosaic_id: Uint64Dto, direction: u8, delta: Uint64Dto) -> EmbeddedMosaicSupplyChangeTransactionDto {
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
