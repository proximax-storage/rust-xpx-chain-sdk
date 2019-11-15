#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicSupplyChangeTransactionBodyDto {
    #[serde(rename = "mosaicId")]
    pub mosaic_id: Vec<i32>,
    #[serde(rename = "direction")]
    pub direction: crate::models::MosaicDirectionEnum,
    #[serde(rename = "delta")]
    pub delta: Vec<i32>,
}

impl MosaicSupplyChangeTransactionBodyDto {
    pub fn new(mosaic_id: Vec<i32>, direction: crate::models::MosaicDirectionEnum, delta: Vec<i32>) -> MosaicSupplyChangeTransactionBodyDto {
        MosaicSupplyChangeTransactionBodyDto {
            mosaic_id,
            direction,
            delta,
        }
    }
}


