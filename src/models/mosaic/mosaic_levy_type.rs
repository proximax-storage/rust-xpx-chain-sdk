/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::{convert::TryFrom, fmt};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum MosaicLevyType {
    levyNone,
    levyAbsoluteFee,
    levyPercentileFee,
}

impl MosaicLevyType {
    const UNKNOWN_LEVY_TYPE: &'static str = "Mosaic levy unsupported";

    pub fn value(self) -> u8 {
        self as u8
    }

    pub fn to_bytes(&self) -> [u8; 1] {
        self.value().to_le_bytes()
    }
}

impl fmt::Display for MosaicLevyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl fmt::Debug for MosaicLevyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.value())
    }
}

/// Returns a 'MosaicLevyType' for the given u8 value.
///
/// Throws an Err UNKNOWN_NETWORK_TYPE when the type is unknown.
impl TryFrom<u8> for MosaicLevyType {
    type Error = anyhow::Error;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        use MosaicLevyType::*;
        match v {
            x if x == levyNone.value() => Ok(levyNone),
            x if x == levyAbsoluteFee.value() => Ok(levyAbsoluteFee),
            x if x == levyPercentileFee.value() => Ok(levyPercentileFee),

            _ => Err(anyhow::anyhow!(Self::UNKNOWN_LEVY_TYPE)),
        }
    }
}
