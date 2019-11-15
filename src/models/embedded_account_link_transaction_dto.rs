#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedAccountLinkTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - Public main network. * 0x98 (TEST_NET) - Public test network. * 0x60 (MIJIN) - Private network. * 0x90 (MIJIN_TEST) - Private test network. 
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
    /// The public key of the remote account.
    #[serde(rename = "remoteAccountKey")]
    pub remote_account_key: String,
    #[serde(rename = "action")]
    pub action: crate::models::LinkActionEnum,
}

impl EmbeddedAccountLinkTransactionDto {
    pub fn new(signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, remote_account_key: String, action: crate::models::LinkActionEnum) -> EmbeddedAccountLinkTransactionDto {
        EmbeddedAccountLinkTransactionDto {
            signer,
            version,
            _type,
            max_fee,
            deadline,
            remote_account_key,
            action,
        }
    }
}


