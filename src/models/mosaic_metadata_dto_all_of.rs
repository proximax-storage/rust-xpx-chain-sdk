#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicMetadataDtoAllOf {
    #[serde(rename = "metadataType", skip_serializing_if = "Option::is_none")]
    pub metadata_type: Option<i32>,
    #[serde(rename = "metadataId")]
    pub metadata_id: Vec<i32>,
}

impl MosaicMetadataDtoAllOf {
    pub fn new(metadata_id: Vec<i32>) -> MosaicMetadataDtoAllOf {
        MosaicMetadataDtoAllOf {
            metadata_type: None,
            metadata_id,
        }
    }
}


