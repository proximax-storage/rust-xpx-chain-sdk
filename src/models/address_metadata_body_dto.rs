#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressMetadataBodyDto {
    /// The address in hexadecimal.
    #[serde(rename = "metadataId")]
    pub metadata_id: String,
    #[serde(rename = "metadataType")]
    pub metadata_type: crate::models::MetadataTypeEnum,
    /// The array of metadata modifications.
    #[serde(rename = "modifications")]
    pub modifications: Vec<crate::models::MetadataModificationDto>,
}

impl AddressMetadataBodyDto {
    pub fn new(metadata_id: String, metadata_type: crate::models::MetadataTypeEnum, modifications: Vec<crate::models::MetadataModificationDto>) -> AddressMetadataBodyDto {
        AddressMetadataBodyDto {
            metadata_id,
            metadata_type,
            modifications,
        }
    }
}


