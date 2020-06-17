// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use crate::models::{mosaic::Mosaic, Uint64};

use super::{account_type::AccountLinkTypeEnum, address_model::Address};

/// The 'AccountInfo' structure describes basic information for an account.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    /// The address of the account.
    pub address: Address,

    /// The block height when the address was published.
    pub address_height: Uint64,

    /// The public key of an account can be used to verify signatures of the account.
    /// Only accounts that have already published a transaction have a public key assigned to the account.
    /// Otherwise, the field is 0000000000000000000000000000000000000000000000000000000000000000.
    pub public_key: String,

    /// The block height when the public key was first published.
    pub public_key_height: Uint64,

    /// The account type.
    pub account_type: AccountLinkTypeEnum,

    pub mosaics: Vec<Mosaic>,
}

impl AccountInfo {
    pub fn new(
        address: Address,
        address_height: Uint64,
        public_key: String,
        public_key_height: Uint64,
        account_type: AccountLinkTypeEnum,
        mosaics: Vec<Mosaic>,
    ) -> Self {
        AccountInfo {
            address,
            address_height,
            public_key,
            public_key_height,
            account_type,
            mosaics,
        }
    }
}

impl core::fmt::Display for AccountInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}
