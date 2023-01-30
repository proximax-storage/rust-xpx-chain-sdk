/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::fmt;

use crate::account::Address;

use super::{MosaicId, MosaicLevyType};

/// MosaicLevy  structure describes mosaic Levy.
///
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct MosaicLevy {
    pub r#type: MosaicLevyType,
    pub recipient: Address,
    pub mosaic_id: MosaicId,
    pub fee: u64,
}

impl fmt::Display for MosaicLevy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self)
    }
}
