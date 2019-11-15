#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountLinkTransactionBodyDto {
    /// The public key of the remote account.
    #[serde(rename = "remoteAccountKey")]
    pub remote_account_key: String,
    #[serde(rename = "action")]
    pub action: crate::models::LinkActionEnum,
}

impl AccountLinkTransactionBodyDto {
    pub fn new(remote_account_key: String, action: crate::models::LinkActionEnum) -> AccountLinkTransactionBodyDto {
        AccountLinkTransactionBodyDto {
            remote_account_key,
            action,
        }
    }
}


