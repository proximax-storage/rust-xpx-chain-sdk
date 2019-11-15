#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountDto {
    /// The account unique address in hexadecimal. 
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "addressHeight")]
    pub address_height: Vec<i32>,
    /// The public key of an account can be used to verify signatures of the account. Only accounts that have already published a transaction have a public key assigned to the account. Otherwise, the field is null. 
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "publicKeyHeight")]
    pub public_key_height: Vec<i32>,
    /// The list of mosaics the account owns. The amount is represented in absolute amount. Thus a balance of 123456789 for a mosaic with divisibility 6 (absolute) means the account owns 123.456789 instead. 
    #[serde(rename = "mosaics")]
    pub mosaics: Vec<crate::models::MosaicDto>,
    #[serde(rename = "accountType")]
    pub account_type: crate::models::AccountLinkTypeEnum,
    /// The public key of a linked account. The linked account can use|provide balance for delegated harvesting. 
    #[serde(rename = "linkedAccountKey")]
    pub linked_account_key: String,
}

impl AccountDto {
    pub fn new(address: String, address_height: Vec<i32>, public_key: String, public_key_height: Vec<i32>, mosaics: Vec<crate::models::MosaicDto>, account_type: crate::models::AccountLinkTypeEnum, linked_account_key: String) -> AccountDto {
        AccountDto {
            address,
            address_height,
            public_key,
            public_key_height,
            mosaics,
            account_type,
            linked_account_key,
        }
    }
}


