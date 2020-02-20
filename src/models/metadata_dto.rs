use crate::models::field_dto::FieldDto;

/// MetadataModificationTypeEnum : The type of the metadata modification: * 0 - Add metadata. * 1 - Remove metadata.
/// The type of the metadata modification: * 0 - Add metadata. * 1 - Remove metadata.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MetadataModificationTypeEnum {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,
}

/// MetadataTypeEnum : The type of the metadata: * 1 - Address metadata. * 2 - Mosaic metadata. * 3 - Namespace metadata.
/// The type of the metadata: * 1 - Address metadata. * 2 - Mosaic metadata. * 3 - Namespace metadata.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MetadataTypeEnum {
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,
    #[serde(rename = "3")]
    _3,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataDto {
    #[serde(rename = "metadataType")]
    pub metadata_type: i32,
    #[serde(rename = "fields")]
    pub fields: Vec<FieldDto>,
}

impl MetadataDto {
    pub fn new(metadata_type: i32, fields: Vec<FieldDto>) -> Self {
        MetadataDto {
            metadata_type,
            fields,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataIds {
    #[serde(rename = "metadataIds", skip_serializing_if = "Option::is_none")]
    pub metadata_ids: Option<Vec<String>>,
}

impl MetadataIds {
    pub fn new() -> Self {
        MetadataIds {
            metadata_ids: None,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataModificationDto {
    #[serde(rename = "modificationType")]
    pub modification_type: MetadataModificationTypeEnum,
    /// The key of metadata modification.
    #[serde(rename = "key")]
    pub key: String,
    /// The value of metadata modification.
    #[serde(rename = "value")]
    pub value: String,
}

impl MetadataModificationDto {
    pub fn new(modification_type: MetadataModificationTypeEnum, key: String, value: String) -> Self {
        MetadataModificationDto {
            modification_type,
            key,
            value,
        }
    }
}

