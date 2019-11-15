#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicMetadataInfoDto {
    #[serde(rename = "metadata")]
    pub metadata: crate::models::MosaicMetadataDto,
}

impl MosaicMetadataInfoDto {
    pub fn new(metadata: crate::models::MosaicMetadataDto) -> MosaicMetadataInfoDto {
        MosaicMetadataInfoDto {
            metadata,
        }
    }
}


