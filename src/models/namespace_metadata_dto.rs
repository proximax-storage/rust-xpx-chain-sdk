#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceMetadataDto {
    #[serde(rename = "metadataType")]
    pub metadata_type: i32,
    #[serde(rename = "fields")]
    pub fields: Vec<crate::models::FieldDto>,
    #[serde(rename = "metadataId")]
    pub metadata_id: Vec<i32>,
}

impl NamespaceMetadataDto {
    pub fn new(metadata_type: i32, fields: Vec<crate::models::FieldDto>, metadata_id: Vec<i32>) -> NamespaceMetadataDto {
        NamespaceMetadataDto {
            metadata_type,
            fields,
            metadata_id,
        }
    }
}


