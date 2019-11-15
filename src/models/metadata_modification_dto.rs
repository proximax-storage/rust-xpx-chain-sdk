#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataModificationDto {
    #[serde(rename = "modificationType")]
    pub modification_type: crate::models::MetadataModificationTypeEnum,
    /// The key of metadata modification.
    #[serde(rename = "key")]
    pub key: String,
    /// The value of metadata modification.
    #[serde(rename = "value")]
    pub value: String,
}

impl MetadataModificationDto {
    pub fn new(modification_type: crate::models::MetadataModificationTypeEnum, key: String, value: String) -> MetadataModificationDto {
        MetadataModificationDto {
            modification_type,
            key,
            value,
        }
    }
}


