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


