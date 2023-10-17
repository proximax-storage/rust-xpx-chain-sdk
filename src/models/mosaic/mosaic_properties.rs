/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use super::MosaicPropertyId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MosaicProperty {
    pub id: MosaicPropertyId,
    pub value: u64,
}

/// `MosaicProperties` structure which includes several properties for defining mosaic
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MosaicProperties {
    /// The creator can choose between a definition that allows a mosaic supply change at a later point or an immutable supply.
    /// Allowed values for the property are "true" and "false". The default value is "false".
    ///
    pub supply_mutable: bool,

    /// The creator can choose if the mosaic definition should allow for transfers of the mosaic among accounts other than the creator.
    /// If the property 'transferable' is set to "false", only transfer transactions
    /// having the creator as sender or as recipient can transfer mosaics of that type.
    /// If set to "true" the mosaics can be transferred to and from arbitrary accounts.
    /// Allowed values for the property are thus "true" and "false". The default value is "true".
    ///
    pub transferable: bool,

    /// `divisibility` - divisibility determines up to what decimal place the mosaic can be divided into
    ///
    pub divisibility: u8,

    pub duration: Option<u64>,

    pub optional_properties: Vec<MosaicProperty>,
}

impl MosaicProperties {
    pub(crate) const FLAG_SUPPLY_MUTABLE: u8 = 0x01;

    pub(crate) const FLAG_TRANSFERABLE: u8 = 0x02;

    pub fn create(
        supply_mutable: bool,
        transferable: bool,
        divisibility: u8,
        duration: Option<u64>,
    ) -> Self {
        let mut properties = vec![];
        if duration.is_some() {
            properties.push(MosaicProperty {
                id: MosaicPropertyId::Duration,
                value: duration.unwrap_or_default(),
            });
        }

        MosaicProperties {
            supply_mutable,
            transferable,
            divisibility,
            duration,
            optional_properties: properties,
        }
    }

    /// Get mosaic flag value in number
    ///
    pub fn get_value(&self) -> u8 {
        return (if self.supply_mutable { Self::FLAG_SUPPLY_MUTABLE } else { 0 })
            + (if self.transferable { Self::FLAG_TRANSFERABLE } else { 0 });
    }
}

/// Creates `MosaicFlags` with the default parameters.
impl Default for MosaicProperties {
    fn default() -> Self {
        MosaicProperties {
            supply_mutable: true,
            transferable: true,
            divisibility: 0,
            duration: Default::default(),
            optional_properties: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mosaic::MosaicProperties;

    #[test]
    fn test_should_create_with_static_values() {
        let mosaic_properties = MosaicProperties::create(false, false, 0, None);

        assert_eq!(mosaic_properties.supply_mutable, false);
        assert_eq!(mosaic_properties.transferable, false);
    }

    #[test]
    fn test_should_return_corredt_flags_value() {
        let mosaic_properties = MosaicProperties::create(false, false, 0, None);
        assert_eq!(mosaic_properties.get_value(), 0);

        let mosaic_properties = MosaicProperties::create(true, false, 0, None);
        assert_eq!(mosaic_properties.get_value(), 1);

        let mosaic_properties = MosaicProperties::create(false, true, 0, None);
        assert_eq!(mosaic_properties.get_value(), 2);

        let mosaic_properties = MosaicProperties::create(true, true, 0, None);
        assert_eq!(mosaic_properties.get_value(), 3);
    }
}
