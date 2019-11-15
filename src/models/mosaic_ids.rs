#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicIds {
    /// The array of mosaic identifiers.
    #[serde(rename = "mosaicIds", skip_serializing_if = "Option::is_none")]
    pub mosaic_ids: Option<Vec<String>>,
}

impl MosaicIds {
    pub fn new() -> MosaicIds {
        MosaicIds {
            mosaic_ids: None,
        }
    }
}


