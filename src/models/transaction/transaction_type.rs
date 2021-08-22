/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use serde::{Serialize, Serializer};

pub(crate) const ACCOUNT_PROPERTY_ADDRESS_VERSION: EntityVersion = 1;
pub(crate) const ACCOUNT_PROPERTY_ENTITY_TYPE_VERSION: EntityVersion = 1;
pub(crate) const ACCOUNT_PROPERTY_MOSAIC_VERSION: EntityVersion = 1;
pub(crate) const ADDRESS_ALIAS_VERSION: EntityVersion = 1;
pub(crate) const AGGREGATE_BONDED_VERSION: EntityVersion = 2;
pub(crate) const AGGREGATE_COMPLETED_VERSION: EntityVersion = 2;
pub(crate) const LOCK_VERSION: EntityVersion = 1;
pub(crate) const MODIFY_MULTISIG_VERSION: EntityVersion = 3;
pub(crate) const MOSAIC_ALIAS_VERSION: EntityVersion = 1;
pub(crate) const MOSAIC_DEFINITION_VERSION: EntityVersion = 3;
pub(crate) const MOSAIC_SUPPLY_CHANGE_VERSION: EntityVersion = 2;
pub(crate) const REGISTER_NAMESPACE_VERSION: EntityVersion = 2;
pub(crate) const TRANSFER_VERSION: EntityVersion = 3;
pub(crate) const ADD_EXCHANGE_OFFER_VERSION: EntityVersion = 3;
pub(crate) const EXCHANGE_OFFER_VERSION: EntityVersion = 1;
pub(crate) const REMOVE_EXCHANGE_OFFER_VERSION: EntityVersion = 1;
pub(crate) const METADATA_ADDRESS_VERSION: EntityVersion = 1;
pub(crate) const METADATA_MOSAIC_VERSION: EntityVersion = 1;
pub(crate) const METADATA_NAMESPACE_VERSION: EntityVersion = 1;

//pub(crate) const BLOCKCHAIN_UPGRADE_VERSION: EntityVersion = 1;
//pub(crate) const DRIVE_FILES_REWARD_VERSION: EntityVersion = 1;
//pub(crate) const DRIVE_FILE_SYSTEM_VERSION: EntityVersion = 1;
//pub(crate) const END_DRIVE_VERIFICATION_VERSION: EntityVersion = 1;
//pub(crate) const END_DRIVE_VERSION: EntityVersion = 1;
//pub(crate) const FILES_DEPOSIT_VERSION: EntityVersion = 1;
//pub(crate) const JOIN_TO_DRIVE_VERSION: EntityVersion = 1;
//pub(crate) const LINK_ACCOUNT_VERSION: EntityVersion = 2;
//pub(crate) const MODIFY_CONTRACT_VERSION: EntityVersion = 3;
//pub(crate) const NETWORK_CONFIG_VERSION: EntityVersion = 1;
//pub(crate) const PREPARE_DRIVE_VERSION: EntityVersion = 1;
//pub(crate) const SECRET_LOCK_VERSION: EntityVersion = 1;
//pub(crate) const SECRET_PROOF_VERSION: EntityVersion = 1;
//pub(crate) const START_DRIVE_VERIFICATION_VERSION: EntityVersion = 1;

pub(crate) type EntityVersion = u32;

/// The entity type:
/// * 0x4141 (16705 decimal) - Aggregate Complete Transaction.
/// * 0x4148 (16712 decimal) - Hash Lock Transaction.
/// * 0x414C (16716 decimal) - Account Link Transaction.
/// * 0x414D (16717 decimal) - Mosaic Definition Transaction.
/// * 0x414E (16718 decimal) - Register Namespace Transaction.
/// * 0x4150 (16720 decimal) - Account Properties Address Transaction.
/// * 0x4152 (16722 decimal) - Secret Lock Transaction.
/// * 0x4154 (16724 decimal) - Transfer Transaction.
/// * 0x413D (16701 decimal) - Modify Metadata Address Transaction.
/// * 0x423D (16957 decimal) - Modify Metadata Mosaic Transaction.
/// * 0x433D (17213 decimal) - Modify Metadata Namespace Transaction.
/// * 0x4158 (16728 decimal) - Blockchain Upgrade Transaction.
/// * 0x4159 (16729 decimal) - Network Config Transaction.
/// * 0x4241 (16961 decimal) - Aggregate Bonded Transaction.
/// * 0x424D (16973 decimal) - Mosaic Supply Change Transaction.
/// * 0x424E (16974 decimal) - Address Alias Transaction.
/// * 0x4250 (16976 decimal) - Account Properties Mosaic Transaction.
/// * 0x4252 (16978 decimal) - Secret Proof Transaction.
/// * 0x434E (17230 decimal) - Mosaic Alias Transaction.
/// * 0x4350 (17232 decimal) - Account Properties Entity Type Transaction.
/// * 0x8043 (32835 decimal) - Nemesis block.
/// * 0x8143 (33091 decimal) - Regular block.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[repr(u16)]
pub enum TransactionType {
    BlockchainUpgrade = 0x4158,
    NetworkConfigEntityType = 0x4159,
    MosaicAlias = 0x434E,
    MosaicDefinition = 0x414D,
    MosaicSupplyChange = 0x424D,
    NamespaceRegistration = 0x414E,
    AddExchangeOffer = 0x415D,
    ExchangeOffer = 0x425D,
    RemoveExchangeOffer = 0x435D,
    AddressAlias = 0x424E,
    Transfer = 0x4154,
    ModifyMetadataAddress = 0x413D,
    ModifyMetadataMosaic = 0x423D,
    ModifyMetadataNamespace = 0x433D,
    ModifyMultisigAccount = 0x4155,
    AggregateComplete = 0x4141,
    AggregateBonded = 0x4241,
    Lock = 0x4148,
    AccountRestrictionAddress = 0x4150,
    AccountRestrictionMosaic = 0x4250,
    AccountRestrictionEntity = 0x4350,
    SecretLock = 0x4152,
    SecretProof = 0x4252,
    AccountLink = 0x414C,
    NemesisBlock = 0x8043,
    Block = 0x8143,
    EntityTypeUnknown,
}

impl TransactionType {
    pub fn value(self) -> u16 {
        self as u16
    }

    pub fn to_bytes(self) -> [u8; 2] {
        self.value().to_le_bytes()
    }

    pub fn to_hex(&self) -> String {
        format!("{:#X}", self.value())
    }
}

impl From<u16> for TransactionType {
    fn from(num: u16) -> Self {
        use TransactionType::*;

        match num {
            0x4141 => AggregateComplete,
            0x4148 => Lock,
            0x414C => AccountLink,
            0x414D => MosaicDefinition,
            0x414E => NamespaceRegistration,
            0x415D => AddExchangeOffer,
            0x425D => ExchangeOffer,
            0x435D => RemoveExchangeOffer,
            0x4150 => AccountRestrictionAddress,
            0x4152 => SecretLock,
            0x4154 => Transfer,
            0x4155 => ModifyMultisigAccount,
            0x413D => ModifyMetadataAddress,
            0x423D => ModifyMetadataMosaic,
            0x433D => ModifyMetadataNamespace,
            0x4158 => BlockchainUpgrade,
            0x4159 => NetworkConfigEntityType,
            0x4241 => AggregateBonded,
            0x424D => MosaicSupplyChange,
            0x424E => AddressAlias,
            0x4250 => AccountRestrictionMosaic,
            0x4252 => SecretProof,
            0x434E => MosaicAlias,
            0x4350 => AccountRestrictionEntity,
            0x8043 => NemesisBlock,
            0x8143 => Block,

            _ => EntityTypeUnknown,
        }
    }
}

impl core::fmt::Display for TransactionType {
    fn fmt(&self, e: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(e, "{:?}", &self)
    }
}

impl Serialize for TransactionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_u16(self.value())
    }
}
