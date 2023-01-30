/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::account::{Address, PublicAccount};
use crate::api::Uint64Dto;
use crate::mosaic::MosaicRichList;

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct MosaicRichListDTO {
    pub address: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    pub amount: Uint64Dto,
}

impl MosaicRichListDTO {
    pub fn compact(&self) -> MosaicRichList {
        let amount = self.amount.compact();
        let address = Address::from_encoded(&self.address).unwrap_or_default();
        let account = PublicAccount::from_public_key(&self.public_key, address.network_type)
            .unwrap_or_default();

        MosaicRichList { account, amount }
    }
}
