/// BalanceChangeReceiptDto : The invisible state change changed an account balance.



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BalanceChangeReceiptDto {
    /// The version of the receipt.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::ReceiptTypeEnum,
    /// The target account public key.
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "mosaicId")]
    pub mosaic_id: Vec<i32>,
    #[serde(rename = "amount")]
    pub amount: Vec<i32>,
}

impl BalanceChangeReceiptDto {
    /// The invisible state change changed an account balance.
    pub fn new(version: i32, _type: crate::models::ReceiptTypeEnum, account: String, mosaic_id: Vec<i32>, amount: Vec<i32>) -> BalanceChangeReceiptDto {
        BalanceChangeReceiptDto {
            version,
            _type,
            account,
            mosaic_id,
            amount,
        }
    }
}


