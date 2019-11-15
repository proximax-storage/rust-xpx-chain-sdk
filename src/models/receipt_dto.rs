#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ReceiptDto {
    /// The version of the receipt.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::ReceiptTypeEnum,
}

impl ReceiptDto {
    pub fn new(version: i32, _type: crate::models::ReceiptTypeEnum) -> ReceiptDto {
        ReceiptDto {
            version,
            _type,
        }
    }
}


