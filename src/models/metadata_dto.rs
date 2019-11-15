#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataDto {
    #[serde(rename = "metadataType")]
    pub metadata_type: i32,
    #[serde(rename = "fields")]
    pub fields: Vec<crate::models::FieldDto>,
}

impl MetadataDto {
    pub fn new(metadata_type: i32, fields: Vec<crate::models::FieldDto>) -> MetadataDto {
        MetadataDto {
            metadata_type,
            fields,
        }
    }
}


