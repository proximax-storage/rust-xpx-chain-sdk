/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use num_enum::IntoPrimitive;

use crate::errors_const::ERR_INVALID_MOSAIC_PROPERTY_ID;

/// The mosaic propery id means:
/// * 0 - MosaicFlags
/// * 1 - Divisibility
/// * 2 - Duration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, IntoPrimitive)]
#[repr(u8)]
pub enum MosaicPropertyId {
    MosaicFlags,
    Divisibility,
    Duration,
}

impl MosaicPropertyId {
    pub fn value(self) -> u8 {
        self.into()
    }
}

impl From<u8> for MosaicPropertyId {
    fn from(id: u8) -> Self {
        use MosaicPropertyId::*;
        assert!(id <= 2, "{}", ERR_INVALID_MOSAIC_PROPERTY_ID);
        match id {
            1 => Divisibility,
            2 => Duration,
            _ => MosaicFlags,
        }
    }
}
