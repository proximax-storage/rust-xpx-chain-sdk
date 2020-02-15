use crate::models::Uint64;

use super::account_type::AccountLinkTypeEnum;
use super::address_model::Address;
use crate::models::mosaic::Mosaic;

/// The 'AccountInfo' structure describes basic information for an account.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    /// The address of the account.
    pub address: Address,

    /// The block height when the address was published.
    pub address_height: Uint64,

    /// The public key of the account.
    pub public_key: String,

    /// The block height when the public key was first published.
    pub public_key_height: Uint64,

    /// The account type.
    pub account_type: AccountLinkTypeEnum,

    pub mosaics: Vec<Mosaic>,
}

impl AccountInfo {
    pub fn new(address: Address, address_height: Uint64,
               public_key: String, public_key_height: Uint64,
               account_type: AccountLinkTypeEnum, mosaics: Vec<Mosaic>
    ) -> Self {
        AccountInfo {
            address,
            address_height,
            public_key,
            public_key_height,
            account_type,
            mosaics
        }
    }
}

impl<'a> core::fmt::Display for AccountInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f, "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}
