/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::{fmt, ops::Deref};

/// `TransactionVersion` struct containing transaction version constants.
///
/// Transaction format versions are defined in sirius-server in each transaction's plugin source code.
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct TransactionVersion(pub u8);

impl TransactionVersion {
    /// Transfer Transaction transaction version.
    ///
    pub const TRANSFER: TransactionVersion = TransactionVersion(3);

    /// Register namespace transaction version.
    ///
    pub const NAMESPACE_REGISTRATION: TransactionVersion = TransactionVersion(2);

    /// Mosaic definition transaction version.
    ///
    pub const MOSAIC_DEFINITION: TransactionVersion = TransactionVersion(3);

    /// Mosaic supply change transaction version.
    ///
    pub const MOSAIC_SUPPLY_CHANGE: TransactionVersion = TransactionVersion(2);

    /// Modify multisig account transaction version.
    ///
    pub const MULTISIG_ACCOUNT_MODIFICATION: TransactionVersion = TransactionVersion(3);

    /// Aggregate complete transaction version.
    ///
    pub const AGGREGATE_COMPLETE: TransactionVersion = TransactionVersion(3);

    /// Aggregate bonded transaction version.
    ///
    pub const AGGREGATE_BONDED: TransactionVersion = TransactionVersion(3);

    /// Lock transaction version.
    ///
    pub const HASH_LOCK: TransactionVersion = TransactionVersion(1);

    /// Secret Lock transaction version.
    ///
    pub const SECRET_LOCK: TransactionVersion = TransactionVersion(1);

    /// Secret Proof transaction version.
    ///
    pub const SECRET_PROOF: TransactionVersion = TransactionVersion(1);

    /// Address Alias transaction version.
    ///
    pub const ADDRESS_ALIAS: TransactionVersion = TransactionVersion(1);

    /// Mosaic Alias transaction version.
    ///
    pub const MOSAIC_ALIAS: TransactionVersion = TransactionVersion(1);

    /// Account Restriction mosaic transaction version.
    ///
    pub const MODIFY_ACCOUNT_RESTRICTION_MOSAIC: TransactionVersion = TransactionVersion(1);

    /// Account Restriction address transaction version.
    ///
    pub const MODIFY_ACCOUNT_RESTRICTION_ADDRESS: TransactionVersion = TransactionVersion(1);

    /// Account Restriction mosaic transaction version.
    ///
    pub const ACCOUNT_MOSAIC_RESTRICTION: TransactionVersion = TransactionVersion(1);

    /// Account Restriction operation transaction version.
    ///
    pub const MODIFY_ACCOUNT_RESTRICTION_ENTITY_TYPE: TransactionVersion = TransactionVersion(1);

    /// Link account transaction version.
    ///
    pub const ACCOUNT_KEY_LINK: TransactionVersion = TransactionVersion(2);

    /// Modify metadata transactions version.
    ///
    pub const MODIFY_METADATA: TransactionVersion = TransactionVersion(1);

    /// Account metadata transaction version.
    ///
    pub const ACCOUNT_METADATA: TransactionVersion = TransactionVersion(1);

    /// Mosaic metadata transaction version.
    ///
    pub const MOSAIC_METADATA: TransactionVersion = TransactionVersion(1);

    /// Namespace metadata transaction version.
    ///
    pub const NAMESPACE_METADATA: TransactionVersion = TransactionVersion(1);

    /// Modify account metadata nem transactions version.
    ///
    pub const ACCOUNT_METADATA_V2: TransactionVersion = TransactionVersion(1);

    /// Modify mosaic metadata nem transactions version.
    ///
    pub const MOSAIC_METADATA_V2: TransactionVersion = TransactionVersion(1);

    /// Modify namespace metadata nem transactions version.
    ///
    pub const NAMESPACE_METADATA_V2: TransactionVersion = TransactionVersion(1);

    /// Modify mosaic modify levy transactions version.
    ///
    pub const MOSAIC_MODIFY_LEVY: TransactionVersion = TransactionVersion(1);

    /// Modify remove mosaic levy transactions version.
    ///
    pub const MOSAIC_REMOVE_LEVY: TransactionVersion = TransactionVersion(1);

    /// Chain configuration transaction version.
    ///
    pub const CHAIN_CONFIG: TransactionVersion = TransactionVersion(1);

    /// Chain upgrade transaction version.
    ///
    pub const CHAIN_UPGRADE: TransactionVersion = TransactionVersion(1);

    /// Add exchange transaction version.
    ///
    pub const ADD_EXCHANGE_OFFER: TransactionVersion = TransactionVersion(4);

    /// Exchange transaction version.
    ///
    pub const EXCHANGE_OFFER: TransactionVersion = TransactionVersion(2);

    /// Remove exchange transaction version.
    ///
    pub const REMOVE_EXCHANGE_OFFER: TransactionVersion = TransactionVersion(2);

    pub fn to_bytes(&self) -> [u8; 1] {
        self.to_le_bytes()
    }
}

impl Deref for TransactionVersion {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for TransactionVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
