#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AnnounceTransactionInfoDto {
    #[serde(rename = "message")]
    pub message: String,
}

impl AnnounceTransactionInfoDto {
    pub fn new(message: String) -> AnnounceTransactionInfoDto {
        AnnounceTransactionInfoDto {
            message,
        }
    }
}
