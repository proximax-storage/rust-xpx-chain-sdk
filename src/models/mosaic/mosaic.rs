/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use anyhow::Result;
use serde::Deserialize;

use {::std::fmt, num_enum::IntoPrimitive};

use crate::transaction::Amount;

use super::{
    internally::{
        PRX_XPX_U64, XPX_DIVISIBILITY, XPX_MAX_RELATIVE_VALUE, XPX_MAX_VALUE, XPX_MIN_VALUE,
    },
    MosaicId, UnresolvedMosaicId,
};

/// mosaic_supply_type :
/// The supply modification direction:
/// * 0  - Decrease.
/// * 1  - Increase.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, IntoPrimitive)]
#[repr(u8)]
pub enum MosaicSupplyType {
    Decrease,
    Increase,
}

impl MosaicSupplyType {
    pub fn value(self) -> u8 {
        self.into()
    }
}

impl From<u8> for MosaicSupplyType {
    fn from(num: u8) -> Self {
        match num {
            1 => MosaicSupplyType::Increase,
            _ => MosaicSupplyType::Decrease,
        }
    }
}

/// A `Mosaic` describes an instance of a mosaic definition.
/// Mosaics can be transferred by means of a transfer transaction.
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mosaic {
    /// The mosaic id. This can either be of type `MosaicId` or `NamespaceId`.
    pub asset_id: Box<dyn UnresolvedMosaicId>,
    /// The mosaic amount.
    pub amount: Amount,
}

impl Mosaic {
    /// Determines the max decimal place to which the mosaic can be divided.
    /// The divisibility must be in the range of 0 and 6.
    ///
    pub const MAX_DIVISIBILITY: u8 = 6;

    pub const MIN_AMOUNT: u64 = 0;

    pub const MAX_AMOUNT_ABSOLUTE: u64 = 9_000_000_000_000_000;

    /// Creates a new Mosaic with the given `UnresolvedMosaicId` with the given amount.
    ///
    /// The quantity is always given in smallest units for the mosaic. For example, if it has a
    /// divisibility of 3 the quantity is given in millis.
    pub fn create<I: UnresolvedMosaicId + 'static>(asset_id: I, amount: Amount) -> Result<Self> {
        ensure!(
			amount <= Self::MAX_AMOUNT_ABSOLUTE,
			format!(
				"Invalid amount {}, the amount must be in the range of {} and {} atomic units.",
				amount,
				Self::MIN_AMOUNT,
				Self::MAX_AMOUNT_ABSOLUTE
			)
		);

        Ok(Self { asset_id: Box::new(asset_id), amount })
    }

    /// Create `Mosaic` with relative amount.
    ///
    /// # Info
    ///
    /// Mosaic units in Symbol are defined as absolute amounts.
    /// To get an absolute amount, multiply the number of assets you want to send by 10 pow(divisibility).
    /// For example, if the mosaic had divisibility 2, to send 10 units (relative) you should define 1000 (absolute) instead.
    ///
    pub fn create_relative<I: 'static + UnresolvedMosaicId>(
        id: I,
        amount: Amount,
        divisibility: u8,
    ) -> Result<Self> {
        ensure!(
			divisibility <= Self::MAX_DIVISIBILITY,
			format!(
				"Invalid divisibility {}, the divisibility must be in the range of 0 and 6.",
				divisibility
			)
		);

        let pow_divisibility = 10_u64.pow(divisibility as u32);

        let max_amount_relative = Self::MAX_AMOUNT_ABSOLUTE / pow_divisibility;
        ensure!(
			amount <= max_amount_relative,
			format!(
				"Invalid amount {}, the relative amount must be in the range of {} and {} units.",
				amount,
				Self::MIN_AMOUNT,
				max_amount_relative
			)
		);

        Self::create(id, amount * pow_divisibility)
    }

    pub fn xpx(amount: Amount) -> Self {
        assert!(amount <= XPX_MAX_VALUE, "Maximum xpx value must be {}", XPX_MAX_VALUE);

        assert!(amount > 0, "Minimum xpx value must be {}", XPX_MIN_VALUE);

        let xpx_mosaic_id = Box::new(MosaicId::from(PRX_XPX_U64));

        Self { asset_id: xpx_mosaic_id, amount }
    }

    pub fn xpx_relative(amount: Amount) -> Self {
        assert!(
            amount <= XPX_MAX_RELATIVE_VALUE,
            "Maximum xpx relative value must be {}",
            XPX_MAX_RELATIVE_VALUE
        );

        assert!(amount > 0, "Minimum xpx relative value must be {}", XPX_MIN_VALUE);

        Mosaic::xpx(amount * XPX_DIVISIBILITY)
    }
}

impl fmt::Display for Mosaic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct MosaicIds {
    /// The array of mosaic identifiers.
    #[serde(rename = "mosaicIds")]
    pub mosaic_ids: Vec<String>,
}

impl From<Vec<MosaicId>> for MosaicIds {
    fn from(e: Vec<MosaicId>) -> Self {
        let mosaic_ids = e.into_iter().map(|m| m.to_hex()).collect();
        Self { mosaic_ids }
    }
}

#[cfg(test)]
mod tests {
    use crate::mosaic::{Mosaic, MosaicId};

    const LO_HI: [u32; 2] = [3646934825, 3576016193];

    #[test]
    fn test_should_create_with_absolute_amount() {
        let id = MosaicId::from(LO_HI);
        let mosaic = Mosaic::create(id, 1).unwrap();

        assert_eq!(mosaic.asset_id.as_ref(), &id);
        assert_eq!(mosaic.amount, 1);
    }

    #[test]
    fn test_should_create_with_relative_amount() {
        let id = MosaicId::from(LO_HI);
        let mosaic = Mosaic::create_relative(id, 1, 6).unwrap();

        assert_eq!(mosaic.asset_id.as_ref(), &id);
        assert_eq!(mosaic.amount, 1_000_000);
    }

    #[test]
    #[should_panic(
    expected = "Invalid divisibility 8, the divisibility must be in the range of 0 and 6."
    )]
    fn test_try_create_with_relative_should_return_panic() {
        let id = MosaicId::from(LO_HI);
        Mosaic::create_relative(id, 1, 8).unwrap();
    }
}
