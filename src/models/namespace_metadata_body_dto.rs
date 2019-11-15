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


