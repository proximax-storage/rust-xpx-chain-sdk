#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MultisigDto {
    /// The account public key.
    #[serde(rename = "account")]
    pub account: String,
    /// The account address in hexadecimal.
    #[serde(rename = "accountAddress", skip_serializing_if = "Option::is_none")]
    pub account_address: Option<String>,
    /// The number of signatures needed to approve a transaction.
    #[serde(rename = "minApproval")]
    pub min_approval: i32,
    /// The number of signatures needed to remove a cosignatory.
    #[serde(rename = "minRemoval")]
    pub min_removal: i32,
    /// The array of public keys of the cosignatory accounts.
    #[serde(rename = "cosignatories")]
    pub cosignatories: Vec<String>,
    /// The array of multisig accounts where the account is cosignatory.
    #[serde(rename = "multisigAccounts")]
    pub multisig_accounts: Vec<String>,
}

impl MultisigDto {
    pub fn new(account: String, min_approval: i32, min_removal: i32, cosignatories: Vec<String>, multisig_accounts: Vec<String>) -> MultisigDto {
        MultisigDto {
            account,
            account_address: None,
            min_approval,
            min_removal,
            cosignatories,
            multisig_accounts,
        }
    }
}


