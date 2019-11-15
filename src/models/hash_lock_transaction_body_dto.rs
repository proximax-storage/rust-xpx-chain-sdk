#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HashLockTransactionBodyDto {
    #[serde(rename = "mosaic")]
    pub mosaic: crate::models::MosaicDto,
    #[serde(rename = "duration")]
    pub duration: crate::models::MosaicDto,
    /// The aggregate bonded transaction hash that has to be confirmed before unlocking the mosaics. 
    #[serde(rename = "hash")]
    pub hash: String,
}

impl HashLockTransactionBodyDto {
    pub fn new(mosaic: crate::models::MosaicDto, duration: crate::models::MosaicDto, hash: String) -> HashLockTransactionBodyDto {
        HashLockTransactionBodyDto {
            mosaic,
            duration,
            hash,
        }
    }
}


