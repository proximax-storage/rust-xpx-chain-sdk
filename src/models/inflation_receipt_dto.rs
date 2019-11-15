/// InflationReceiptDto : Native currency mosaics were created due to inflation.



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InflationReceiptDto {
    /// The version of the receipt.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::ReceiptTypeEnum,
    #[serde(rename = "mosaicId")]
    pub mosaic_id: Vec<i32>,
    #[serde(rename = "amount")]
    pub amount: Vec<i32>,
}

impl InflationReceiptDto {
    /// Native currency mosaics were created due to inflation.
    pub fn new(version: i32, _type: crate::models::ReceiptTypeEnum, mosaic_id: Vec<i32>, amount: Vec<i32>) -> InflationReceiptDto {
        InflationReceiptDto {
            version,
            _type,
            mosaic_id,
            amount,
        }
    }
}


