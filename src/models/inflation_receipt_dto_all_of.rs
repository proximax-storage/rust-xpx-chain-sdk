#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InflationReceiptDtoAllOf {
    #[serde(rename = "mosaicId")]
    pub mosaic_id: Vec<i32>,
    #[serde(rename = "amount")]
    pub amount: Vec<i32>,
}

impl InflationReceiptDtoAllOf {
    pub fn new(mosaic_id: Vec<i32>, amount: Vec<i32>) -> InflationReceiptDtoAllOf {
        InflationReceiptDtoAllOf {
            mosaic_id,
            amount,
        }
    }
}


