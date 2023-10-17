/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use num_enum::IntoPrimitive;

/// AccountLinkAction :
/// The type of the action:
/// * 0 - Link.
/// * 1 - Unlink.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, IntoPrimitive)]
#[repr(u8)]
pub enum AccountLinkAction {
    AccountLink,
    AccountUnlink,
}

impl AccountLinkAction {
    pub fn value(self) -> u8 {
        self.into()
    }
}

impl From<u8> for AccountLinkAction {
    fn from(num: u8) -> Self {
        match num {
            1 => AccountLinkAction::AccountUnlink,
            _ => AccountLinkAction::AccountLink,
        }
    }
}

impl std::fmt::Display for AccountLinkAction {
    fn fmt(&self, e: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(e, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

/// The alias type:
/// * 0 -  No alias.
/// * 1 -  Mosaic id alias.
/// * 2 -  Addres alias.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, IntoPrimitive)]
#[repr(u8)]
pub enum AliasType {
    NoneAliasType,
    MosaicAliasType,
    AddressAliasType,
}

impl AliasType {
    pub fn value(self) -> u8 {
        self.into()
    }
}

impl From<u8> for AliasType {
    fn from(num: u8) -> Self {
        use AliasType::*;
        match num {
            1 => MosaicAliasType,
            2 => AddressAliasType,
            _ => NoneAliasType,
        }
    }
}

impl std::fmt::Display for AliasType {
    fn fmt(&self, e: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(e, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

/// The alias action:
/// * 0 -  Link alias.
/// * 1 -  Unlink alias.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, IntoPrimitive)]
#[repr(u8)]
pub enum AliasActionType {
    AliasLink,
    AliasUnlink,
}

impl AliasActionType {
    pub fn value(self) -> u8 {
        self.into()
    }
}

impl From<u8> for AliasActionType {
    fn from(num: u8) -> Self {
        match num {
            1 => AliasActionType::AliasUnlink,
            _ => AliasActionType::AliasLink,
        }
    }
}

impl std::fmt::Display for AliasActionType {
    fn fmt(&self, e: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(e, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}
