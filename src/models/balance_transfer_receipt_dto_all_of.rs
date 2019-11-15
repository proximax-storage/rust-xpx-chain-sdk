#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BalanceTransferReceiptDtoAllOf {
    /// The public key of the sender.
    #[serde(rename = "sender")]
    pub sender: String,
    /// The public key of the recipient.
    #[serde(rename = "recipient")]
    pub recipient: String,
    #[serde(rename = "mosaicId")]
    pub mosaic_id: Vec<i32>,
    #[serde(rename = "amount")]
    pub amount: Vec<i32>,
}

impl BalanceTransferReceiptDtoAllOf {
    pub fn new(sender: String, recipient: String, mosaic_id: Vec<i32>, amount: Vec<i32>) -> BalanceTransferReceiptDtoAllOf {
        BalanceTransferReceiptDtoAllOf {
            sender,
            recipient,
            mosaic_id,
            amount,
        }
    }
}


