#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionStatusDto {
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "deadline", skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Vec<i32>>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<Vec<i32>>,
}

impl TransactionStatusDto {
    pub fn new(status: String) -> TransactionStatusDto {
        TransactionStatusDto {
            group: None,
            status,
            hash: None,
            deadline: None,
            height: None,
        }
    }
}


