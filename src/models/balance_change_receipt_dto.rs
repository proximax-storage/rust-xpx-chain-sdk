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
