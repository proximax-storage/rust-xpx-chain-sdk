#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressMetadataDto {
    #[serde(rename = "metadataType")]
    pub metadata_type: i32,
    #[serde(rename = "fields")]
    pub fields: Vec<crate::models::FieldDto>,
    #[serde(rename = "metadataId")]
    pub metadata_id: String,
}

impl AddressMetadataDto {
    pub fn new(metadata_type: i32, fields: Vec<crate::models::FieldDto>, metadata_id: String) -> AddressMetadataDto {
        AddressMetadataDto {
            metadata_type,
            fields,
            metadata_id,
        }
    }
}


