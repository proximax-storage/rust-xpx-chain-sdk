#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressMetadataDtoAllOf {
    #[serde(rename = "metadataId")]
    pub metadata_id: String,
}

impl AddressMetadataDtoAllOf {
    pub fn new(metadata_id: String) -> AddressMetadataDtoAllOf {
        AddressMetadataDtoAllOf {
            metadata_id,
        }
    }
}


