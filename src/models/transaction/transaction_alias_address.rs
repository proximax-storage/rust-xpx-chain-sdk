/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    serde_json::Value,
    std::{any::Any, fmt},
};

use crate::{
    models::{
        account::{Account, Address, PublicAccount},
        alias::AliasActionType,
        asset_id_model::AssetId,
        consts::ADDRESS_SIZE,
        errors_const,
        namespace::NamespaceId,
        network::NetworkType,
    },
    Result,
};

use super::{
    internal::sign_transaction, AbsTransaction, AbstractTransaction, AliasTransaction, Deadline,
    HashValue, SignedTransaction, Transaction, TransactionType, ADDRESS_ALIAS_VERSION,
};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressAliasTransaction {
    pub alias_transaction: AliasTransaction,
    pub address: Address,
}

impl AddressAliasTransaction {
    pub fn new(
        deadline: Deadline,
        address: Address,
        namespace_id: NamespaceId,
        action_type: AliasActionType,
        network_type: NetworkType,
    ) -> Result<Self> {
        ensure!(!address.is_empty(), errors_const::ERR_EMPTY_ADDRESSES);

        ensure!(
            !namespace_id.is_empty(),
            errors_const::ERR_EMPTY_NAMESPACE_ID
        );

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            ADDRESS_ALIAS_VERSION,
            TransactionType::AddressAlias,
            network_type,
        );

        Ok(Self {
            alias_transaction: AliasTransaction {
                abs_transaction: abs_tx,
                action_type,
                namespace_id,
            },
            address,
        })
    }
}

impl AbsTransaction for AddressAliasTransaction {
    fn abs_transaction(&self) -> AbstractTransaction {
        self.alias_transaction.abs_transaction()
    }
}

impl Transaction for AddressAliasTransaction {
    fn size(&self) -> usize {
        self.alias_transaction.size() + ADDRESS_SIZE
    }

    fn as_value(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    fn sign_transaction_with(
        self,
        account: Account,
        generation_hash: HashValue,
    ) -> Result<SignedTransaction> {
        sign_transaction(self, account, generation_hash)
    }

    fn embedded_to_bytes(&self) -> Result<Vec<u8>> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut builder = fb::FlatBufferBuilder::new();

        let address_bytes = self.address.as_bytes();

        let address_vector = builder.create_vector_direct(address_bytes);

        self.alias_transaction
            .embedded_to_bytes(&mut builder, address_vector, ADDRESS_SIZE)
    }

    fn set_aggregate(&mut self, signer: PublicAccount) {
        self.alias_transaction.set_aggregate(signer)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn box_clone(&self) -> Box<dyn Transaction + 'static> {
        Box::new((*self).clone())
    }
}

impl fmt::Display for AddressAliasTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
