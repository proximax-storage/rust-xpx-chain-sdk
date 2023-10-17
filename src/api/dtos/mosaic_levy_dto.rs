/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::account::Address;
use crate::api::Uint64Dto;
use crate::mosaic::{MosaicId, MosaicLevy};

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct MosaicLevyDTO {
    pub r#type: u8,
    pub recipient: String,
    pub mosaic_id: Uint64Dto,
    pub fee: Uint64Dto,
}

impl MosaicLevyDTO {
    pub fn compact(&self) -> anyhow::Result<MosaicLevy> {
        Ok(MosaicLevy {
            r#type: self.r#type.try_into()?,
            recipient: Address::from_encoded(&self.recipient)?,
            mosaic_id: MosaicId::from(self.mosaic_id.compact()),
            fee: self.fee.compact(),
        })
    }
}
