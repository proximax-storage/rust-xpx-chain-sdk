use crate::models::Uint64;

use super::account_type::AccountLinkTypeEnum;
use super::address_model::Address;

/// The 'AccountInfo' structure describes basic information for an account.
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    /// The address of the account.
    #[serde(rename = "address")]
    pub address: Address,

    /// The block height when the address was published.
    #[serde(rename = "addressHeight")]
    pub address_height: Uint64,

    /// The public key of the account.
    #[serde(rename = "publicKey")]
    pub public_key: String,

    /// The block height when the public key was first published.
    #[serde(rename = "publicKeyHeight")]
    pub public_key_height: Uint64,

    /// The account type.
    #[serde(rename = "account_type")]
    pub account_type: AccountLinkTypeEnum,
}

impl AccountInfo {
    pub fn new(address: Address, address_height: Uint64, public_key: String, public_key_height: Uint64, account_type: AccountLinkTypeEnum) -> Self {
        AccountInfo {
            address,
            address_height,
            public_key,
            public_key_height,
            account_type,
        }
    }
}
