/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::models::{mosaic::MosaicPropertyId, Uint64};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MosaicProperty {
    pub id: MosaicPropertyId,
    pub value: Uint64,
}

/// `mosaic_properties` structure which includes several properties for defining mosaic
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MosaicProperties {
    /// `supply_mutable` - is supply of defined mosaic can be changed in future
    ///
    pub supply_mutable: bool,

    /// `transferable` - if this property is set to "false",
    /// only transfer transactions having the creator as sender or as recipient can transfer mosaics of
    /// that type. If set to "true" the mosaics can be transferred to and from arbitrary accounts
    ///
    pub transferable: bool,

    /// `divisibility` - divisibility determines up to what decimal place the mosaic can be divided into
    ///
    pub divisibility: u8,

    pub duration: Uint64,

    pub optional_properties: Vec<MosaicProperty>,
}

impl MosaicProperties {
    pub fn new(
        supply_mutable: bool,
        transferable: bool,
        divisibility: u8,
        duration: Uint64,
    ) -> crate::Result<Self> {
        ensure!(!0 > divisibility, "Divisibility must not be negative.");

        let mut properties = vec![];
        if *duration != 0 {
            properties.push(MosaicProperty {
                id: MosaicPropertyId::Duration,
                value: duration,
            });
        }

        Ok(MosaicProperties {
            supply_mutable,
            transferable,
            divisibility,
            duration,
            optional_properties: properties,
        })
    }
}

/// Creates `MosaicFlags` with the default parameters.
impl Default for MosaicProperties {
    fn default() -> Self {
        MosaicProperties {
            supply_mutable: true,
            transferable: true,
            divisibility: 0,
            duration: Uint64::default(),
            optional_properties: vec![],
        }
    }
}
