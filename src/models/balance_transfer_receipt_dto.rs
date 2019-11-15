/// BalanceTransferReceiptDto : The invisible state change triggered a mosaic transfer.



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BalanceTransferReceiptDto {
    /// The version of the receipt.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::ReceiptTypeEnum,
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

impl BalanceTransferReceiptDto {
    /// The invisible state change triggered a mosaic transfer.
    pub fn new(version: i32, _type: crate::models::ReceiptTypeEnum, sender: String, recipient: String, mosaic_id: Vec<i32>, amount: Vec<i32>) -> BalanceTransferReceiptDto {
        BalanceTransferReceiptDto {
            version,
            _type,
            sender,
            recipient,
            mosaic_id,
            amount,
        }
    }
}


