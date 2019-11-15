#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressMetadataInfoDto {
    #[serde(rename = "metadata")]
    pub metadata: crate::models::AddressMetadataDto,
}

impl AddressMetadataInfoDto {
    pub fn new(metadata: crate::models::AddressMetadataDto) -> AddressMetadataInfoDto {
        AddressMetadataInfoDto {
            metadata,
        }
    }
}


