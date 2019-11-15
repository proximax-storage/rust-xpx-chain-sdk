#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataIds {
    #[serde(rename = "metadataIds", skip_serializing_if = "Option::is_none")]
    pub metadata_ids: Option<Vec<String>>,
}

impl MetadataIds {
    pub fn new() -> MetadataIds {
        MetadataIds {
            metadata_ids: None,
        }
    }
}


