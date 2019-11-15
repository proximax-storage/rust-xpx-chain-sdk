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


