/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    num_enum::{FromPrimitive, IntoPrimitive},
    serde_repr::*,
};

/// The provided code defines the `TransactionType` enum,
/// which represents different types of transactions in a blockchain system.
/// Each transaction type variant is assigned a decimal value and a hexadecimal value using the repr attribute.
#[derive(
Debug,
Clone,
Copy,
Eq,
PartialEq,
Serialize_repr,
Deserialize_repr,
IntoPrimitive,
FromPrimitive,
strum::Display,
)]
#[repr(u16)]
pub enum TransactionType {
    /// Reserved entity type.
    Reserved = 0,

    /// Blockchain Upgrade transaction type.
    /// Decimal value = 16728
    /// Hex value = 0x4158
    ///
    BlockchainUpgrade = 0x4158,

    /// Network Config transaction type.
    /// Decimal value = 16729
    /// Hex value = 0x4159
    ///
    NetworkConfigEntityType = 0x4159,

    /// Mosaic alias transaction type.
    /// Decimal value = 17230
    /// Hex value = 0x434E
    ///
    MosaicAlias = 0x434E,

    /// Mosaic definition transaction type.
    /// Decimal value = 16717
    /// Hex value = 0x414d
    ///
    MosaicDefinition = 0x414D,

    /// Mosaic supply change transaction.
    /// Decimal value = 16973
    /// Hex value = 0x424d
    ///
    MosaicSupplyChange = 0x424D,

    /// Register namespace transaction type.
    /// Decimal value = 16718
    /// Hex value = 0x414e
    ///
    RegisterNamespace = 0x414E,

    /// Add Exchange Offer transaction type.
    /// Decimal value = 16733
    /// Hex value = 0x415D
    ///
    AddExchangeOffer = 0x415D,

    /// Exchange Offer transaction type.
    /// Decimal value = 16989
    /// Hex value = 0x425D
    ///
    ExchangeOffer = 0x425D,

    /// Remove Exchange Offer transaction type.
    /// Decimal value = 17245
    /// Hex value = 0x435D
    ///
    RemoveExchangeOffer = 0x435D,

    /// Address alias transaction type.
    /// Decimal value = 16974
    /// Hex value = 0x424E
    ///
    AddressAlias = 0x424E,

    /// Transfer Transaction transaction type.
    /// Decimal value = 16724
    /// Hex value = 0x4154
    ///
    Transfer = 0x4154,

    /// Address Metadata transaction type.
    /// Decimal value = 16701
    /// Hex value = 0x413D
    ///
    ModifyMetadataAddress = 0x413D,

    /// Mosaic Metadata transaction type.
    /// Decimal value = 16957
    /// Hex value = 0x423D
    ///
    ModifyMetadataMosaic = 0x423D,

    /// Namespace Metadata transaction type.
    /// Decimal value = 17213
    /// Hex value = 0x433D
    ///
    ModifyMetadataNamespace = 0x433D,

    /// Modify multisig account transaction type.
    /// Decimal value = 16725
    /// Hex value = 0x4155
    ///
    MultisigAccountModify = 0x4155,

    /// Aggregate complete transaction type.
    /// Decimal value = 16705
    /// Hex value = 0x4141
    ///
    AggregateComplete = 0x4141,

    /// Aggregate bonded transaction type.
    /// Decimal value = 16961
    /// Hex value = 0x4241
    ///
    AggregateBonded = 0x4241,

    /// Lock transaction type.
    /// Decimal value = 16712
    /// Hex value = 0x4148
    ///
    HashLock = 0x4148,

    /// Account restriction address transaction type.
    /// Decimal value = 16720
    /// Hex value = 0x4150
    ///
    AccountRestrictionAddress = 0x4150,

    /// Account restriction mosaic transaction type.
    /// Decimal value = 16976
    /// Hex value = 0x4250
    ///
    AccountRestrictionMosaic = 0x4250,

    /// Account restriction operation transaction type.
    /// Decimal value = 17232
    /// Hex value = 0x4350
    ///
    AccountRestrictionOperation = 0x4350,

    /// Secret Lock Transaction type.
    /// Decimal value = 16722
    /// Hex value = 0x4152
    ///
    SecretLock = 0x4152,

    /// Secret Proof transaction type.
    /// Decimal value = 16978
    /// Hex value = 0x4252
    ///
    SecretProof = 0x4252,

    /// Link account transaction type.
    /// Decimal value = 16716
    /// Hex value = 0x414C
    ///
    AccountLink = 0x414C,

    /// Nemesis Block transaction type.
    /// Decimal value = 32835
    /// Hex value = 0x8043
    ///
    NemesisBlock = 0x8043,

    /// Regular block transaction type.
    /// Decimal value = 33091
    /// Hex value = 0x8143
    ///
    Block = 0x8143,

    /// Modify account metadata transaction type - NEM
    /// Decimal value = 16703
    /// Hex value = 0x413F
    ///
    AccountMetadataV2 = 0x413F,

    /// Modify mosaic metadata transaction type - NEM
    /// Decimal value = 16959
    /// Hex value = 0x423F
    ///
    MosaicMetadataV2 = 0x423F,

    /// Modify namespace metadata transaction type - NEM
    /// Decimal value = 17215
    /// Hex value = 0x433F
    ///
    NamespaceMetadataV2 = 0x433F,

    /// Modify mosaic levy transaction type.
    /// Decimal value = 17229
    /// Hex value = 0x434D
    ///
    ModifyMosaicLevy = 0x434D,

    /// Remove mosaic levy transaction type.
    /// Decimal value = 17485
    /// Hex value = 0x444D
    ///
    RemoveMosaicLevy = 0x444D,

    #[num_enum(default)]
    EntityTypeUnknown,
}

impl TransactionType {
    /// Returns the decimal value of the transaction type.
    pub fn value(self) -> u16 {
        self.into()
    }

    /// Converts the transaction type into a little-endian byte array.
    pub fn to_bytes(self) -> [u8; 2] {
        self.value().to_le_bytes()
    }

    /// Returns the hexadecimal representation of the transaction type.
    pub fn to_hex(&self) -> String {
        format!("{:#X}", self.value())
    }
}

impl From<TransactionType> for [u8; 2] {
    /// Converts the `TransactionType` into a byte array.
    fn from(value: TransactionType) -> Self {
        value.to_bytes()
    }
}

#[cfg(test)]
mod tests {
    use crate::transaction::TransactionType;

    #[test]
    fn test_should_match_the_specification() {
        assert_eq!(TransactionType::BlockchainUpgrade, 0x4158.into());
        assert_eq!(TransactionType::NetworkConfigEntityType, 0x4159.into());
        assert_eq!(TransactionType::MosaicAlias, 0x434E.into());
        assert_eq!(TransactionType::MosaicDefinition, 0x414d.into());
        assert_eq!(TransactionType::MosaicSupplyChange, 0x424d.into());
        assert_eq!(TransactionType::RegisterNamespace, 0x414e.into());
        assert_eq!(TransactionType::AddExchangeOffer, 0x415D.into());
        assert_eq!(TransactionType::ExchangeOffer, 0x425D.into());
        assert_eq!(TransactionType::RemoveExchangeOffer, 0x435D.into());
        assert_eq!(TransactionType::AddressAlias, 0x424E.into());
        assert_eq!(TransactionType::Transfer, 0x4154.into());
        assert_eq!(TransactionType::ModifyMetadataAddress, 0x413D.into());
        assert_eq!(TransactionType::ModifyMetadataMosaic, 0x423D.into());
        assert_eq!(TransactionType::ModifyMetadataNamespace, 0x433D.into());
        assert_eq!(TransactionType::MultisigAccountModify, 0x4155.into());
        assert_eq!(TransactionType::AggregateComplete, 0x4141.into());
        assert_eq!(TransactionType::AggregateBonded, 0x4241.into());
        assert_eq!(TransactionType::HashLock, 0x4148.into());
        assert_eq!(TransactionType::AccountRestrictionAddress, 0x4150.into());
        assert_eq!(TransactionType::AccountRestrictionMosaic, 0x4250.into());
        assert_eq!(TransactionType::AccountRestrictionOperation, 0x4350.into());
        assert_eq!(TransactionType::SecretLock, 0x4152.into());
        assert_eq!(TransactionType::SecretProof, 0x4252.into());
        assert_eq!(TransactionType::AccountLink, 0x414C.into());
        assert_eq!(TransactionType::NemesisBlock, 0x8043.into());
        assert_eq!(TransactionType::Block, 0x8143.into());
        assert_eq!(TransactionType::AccountMetadataV2, 0x413F.into());
        assert_eq!(TransactionType::MosaicMetadataV2, 0x423F.into());
        assert_eq!(TransactionType::NamespaceMetadataV2, 0x433F.into());
        assert_eq!(TransactionType::ModifyMosaicLevy, 0x434D.into());
        assert_eq!(TransactionType::RemoveMosaicLevy, 0x444D.into());
    }
}
