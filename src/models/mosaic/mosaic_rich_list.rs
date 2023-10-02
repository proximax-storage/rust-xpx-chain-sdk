/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::fmt;

use crate::account::PublicAccount;

/// `MosaicRichList` structure describes mosaic properties.
///
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct MosaicRichList {
    pub account: PublicAccount,
    pub amount: u64,
}

impl fmt::Display for MosaicRichList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self)
    }
}
