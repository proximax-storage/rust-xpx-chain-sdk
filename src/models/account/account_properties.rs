/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    ::std::ops::BitAnd,
    serde_repr::{Deserialize_repr, Serialize_repr},
};

use crate::{AssetId, mosaic::MosaicId, transaction::TransactionType};

use super::Address;

/// The account properties modification type:
///* 0 - Add property.
///* 1 - Remove property.
#[derive(Debug, Clone, Copy, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AccountPropertiesModificationType {
    AddProperty,
    RemoveProperty,
    AccountPropertiesModificationTypeUnknown,
}

impl AccountPropertiesModificationType {
    pub fn value(self) -> u8 {
        self as u8
    }
}

impl From<u8> for AccountPropertiesModificationType {
    fn from(num: u8) -> Self {
        use AccountPropertiesModificationType::*;
        match num {
            0x00 => AddProperty,
            0x01 => RemoveProperty,
            _ => AccountPropertiesModificationTypeUnknown,
        }
    }
}

/// Account property type
/// * 0x01 (1 decimal) - The property type only allows receiving transactions from an address.
/// * 0x02 (2 decimal) - The property type only allows receiving transactions containing a mosaic id.
/// * 0x04 (4 decimal) - The property type only allows sending transactions with a given transaction type.
/// * 0x05 (5 decimal) - Property type sentinel.
/// * 0x81 (129 decimal) - The property type blocks receiving transactions from an address.
/// * 0x82 (130 decimal) - The property type blocks receiving transactions containing a mosaic id.
/// * 0x84 (132 decimal) -  The property type blocks sending transactions with a given transaction type.
#[derive(Debug, Clone, Copy, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AccountPropertyType {
    AllowAddress = 0x01,
    AllowMosaic = 0x02,
    AllowTransaction = 0x04,
    BlockAddress = 0x80 + 0x01,
    BlockMosaic = 0x80 + 0x02,
    BlockTransaction = 0x80 + 0x04,
    AccountPropertyTypeUnknown,
}

impl AccountPropertyType {
    pub fn value(self) -> u8 {
        self as u8
    }
}

impl From<u8> for AccountPropertyType {
    fn from(num: u8) -> Self {
        use AccountPropertyType::*;
        match num {
            0x01 => AllowAddress,
            0x02 => AllowMosaic,
            0x04 => AllowTransaction,
            0x81 => BlockAddress,
            0x82 => BlockMosaic,
            0x84 => BlockTransaction,

            _ => AccountPropertyTypeUnknown,
        }
    }
}

impl BitAnd for AccountPropertyType {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a & b`
    fn bitand(self, rhs: Self) -> Self::Output {
        AccountPropertyType::from(self.value() & rhs.value())
    }
}

impl core::fmt::Display for AccountPropertyType {
    fn fmt(&self, e: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(e, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AccountProperties {
    pub address: Address,
    pub allowed_addresses: Vec<Address>,
    pub allowed_mosaic_id: Vec<MosaicId>,
    pub allowed_entity_types: Vec<TransactionType>,
    pub blocked_addresses: Vec<Address>,
    pub blocked_mosaic_id: Vec<MosaicId>,
    pub blocked_entity_types: Vec<TransactionType>,
}

impl core::fmt::Display for AccountProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AccountPropertiesAddressModification {
    pub modification_type: AccountPropertiesModificationType,
    pub address: Address,
}

impl AccountPropertiesAddressModification {
    pub fn new(modification_type: AccountPropertiesModificationType, address: Address) -> Self {
        Self {
            modification_type,
            address,
        }
    }
}

impl core::fmt::Display for AccountPropertiesAddressModification {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AccountPropertiesMosaicModification {
    pub modification_type: AccountPropertiesModificationType,
    pub asset_id: Box<dyn AssetId>,
}

impl AccountPropertiesMosaicModification {
    pub fn new(
        modification_type: AccountPropertiesModificationType,
        asset_id: impl AssetId + 'static,
    ) -> Self {
        Self {
            modification_type,
            asset_id: Box::new(asset_id),
        }
    }
}

impl core::fmt::Display for AccountPropertiesMosaicModification {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AccountPropertiesEntityTypeModification {
    pub modification_type: AccountPropertiesModificationType,
    pub transaction_type: TransactionType,
}

impl AccountPropertiesEntityTypeModification {
    pub fn new(
        modification_type: AccountPropertiesModificationType,
        transaction_type: TransactionType,
    ) -> Self {
        Self {
            modification_type,
            transaction_type,
        }
    }
}

impl core::fmt::Display for AccountPropertiesEntityTypeModification {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}
