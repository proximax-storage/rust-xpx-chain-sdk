#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicNamesDto {
    #[serde(rename = "mosaicId")]
    pub mosaic_id: Vec<i32>,
    /// The mosaic linked namespace names.
    #[serde(rename = "names")]
    pub names: Vec<String>,
}

impl MosaicNamesDto {
    pub fn new(mosaic_id: Vec<i32>, names: Vec<String>) -> MosaicNamesDto {
        MosaicNamesDto {
            mosaic_id,
            names,
        }
    }
}


