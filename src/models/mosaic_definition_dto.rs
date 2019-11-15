#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicDefinitionDto {
    #[serde(rename = "mosaicId")]
    pub mosaic_id: Vec<i32>,
    #[serde(rename = "supply")]
    pub supply: Vec<i32>,
    #[serde(rename = "height")]
    pub height: Vec<i32>,
    /// The public key of the mosaic owner.
    #[serde(rename = "owner")]
    pub owner: String,
    /// The number of definitions for the same mosaic.
    #[serde(rename = "revision")]
    pub revision: i32,
    #[serde(rename = "properties")]
    pub properties: Vec<crate::models::MosaicPropertyDto>,
}

impl MosaicDefinitionDto {
    pub fn new(mosaic_id: Vec<i32>, supply: Vec<i32>, height: Vec<i32>, owner: String, revision: i32, properties: Vec<crate::models::MosaicPropertyDto>) -> MosaicDefinitionDto {
        MosaicDefinitionDto {
            mosaic_id,
            supply,
            height,
            owner,
            revision,
            properties,
        }
    }
}


