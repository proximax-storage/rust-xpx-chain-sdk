#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BalanceChangeReceiptDtoAllOf {
    /// The target account public key.
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "mosaicId")]
    pub mosaic_id: Vec<i32>,
    #[serde(rename = "amount")]
    pub amount: Vec<i32>,
}

impl BalanceChangeReceiptDtoAllOf {
    pub fn new(account: String, mosaic_id: Vec<i32>, amount: Vec<i32>) -> BalanceChangeReceiptDtoAllOf {
        BalanceChangeReceiptDtoAllOf {
            account,
            mosaic_id,
            amount,
        }
    }
}


