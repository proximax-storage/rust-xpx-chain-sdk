/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    ::std::ops::BitAnd,
    num_enum::IntoPrimitive,
    serde_repr::{Deserialize_repr, Serialize_repr},
};

use super::Address;
use crate::{mosaic::MosaicId, transaction::EntityTypeEnum, AssetId};

pub type PropertyModificationType = u8;

pub const ADD_PROPERTY: PropertyModificationType = 0;
pub const REMOVE_PROPERTY: PropertyModificationType = 1;

/// Account property type
/// 0x01	The property type is an address.
/// 0x02	The property type is mosaic id.
/// 0x04	The property type is a transaction type.
/// 0x05	Property type sentinel.
/// 0x80 + type	The property is interpreted as a blocking operation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize_repr, Deserialize_repr, IntoPrimitive)]
#[repr(u8)]
pub enum AccountPropertyType {
    AllowAddress = 0x01,
    AllowMosaic = 0x02,
    AllowTransaction = 0x04,
    BlockAddress = 0x80 + 0x01,
    BlockMosaic = 0x80 + 0x02,
    BlockTransaction = 0x80 + 0x04,
    EntityTypeUnknown,
}

impl AccountPropertyType {
    pub fn value(self) -> u8 {
        self.into()
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl From<u8> for AccountPropertyType {
    fn from(num: u8) -> Self {
        match num {
            0x01 => AccountPropertyType::AllowAddress,
            0x02 => AccountPropertyType::AllowMosaic,
            0x04 => AccountPropertyType::AllowTransaction,
            0x81 => AccountPropertyType::BlockAddress,
            0x82 => AccountPropertyType::BlockMosaic,
            0x84 => AccountPropertyType::BlockTransaction,

            _ => AccountPropertyType::EntityTypeUnknown,
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
    pub allowed_entity_types: Vec<EntityTypeEnum>,
    pub blocked_addresses: Vec<Address>,
    pub blocked_mosaic_id: Vec<MosaicId>,
    pub blocked_entity_types: Vec<EntityTypeEnum>,
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
    pub modification_type: PropertyModificationType,
    pub address: Address,
}

impl AccountPropertiesAddressModification {
    pub fn new(modification_type: PropertyModificationType, address: Address) -> Self {
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
    pub modification_type: PropertyModificationType,
    pub asset_id: Box<dyn AssetId>,
}

impl AccountPropertiesMosaicModification {
    pub fn new(
        modification_type: PropertyModificationType,
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
    pub modification_type: PropertyModificationType,
    pub transaction_type: EntityTypeEnum,
}

impl AccountPropertiesEntityTypeModification {
    pub fn new(
        modification_type: PropertyModificationType,
        transaction_type: EntityTypeEnum,
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
