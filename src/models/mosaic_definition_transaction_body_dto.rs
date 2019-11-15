#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicDefinitionTransactionBodyDto {
    /// Random nonce used to generate the mosaic id.
    #[serde(rename = "mosaicNonce")]
    pub mosaic_nonce: i32,
    #[serde(rename = "mosaicId")]
    pub mosaic_id: Vec<i32>,
    #[serde(rename = "properties")]
    pub properties: Vec<crate::models::MosaicPropertyDto>,
}

impl MosaicDefinitionTransactionBodyDto {
    pub fn new(mosaic_nonce: i32, mosaic_id: Vec<i32>, properties: Vec<crate::models::MosaicPropertyDto>) -> MosaicDefinitionTransactionBodyDto {
        MosaicDefinitionTransactionBodyDto {
            mosaic_nonce,
            mosaic_id,
            properties,
        }
    }
}


