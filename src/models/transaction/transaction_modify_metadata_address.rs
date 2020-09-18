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
        consts::ADDRESS_SIZE,
        errors_const,
        metadata::{MetadataModification, MetadataType},
        network::NetworkType,
    },
    Result,
};

use super::{
    internal::sign_transaction, AbsTransaction, AbstractTransaction, Deadline, HashValue,
    ModifyMetadataTransaction, SignedTransaction, Transaction, TransactionType,
    METADATA_ADDRESS_VERSION,
};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataAddressTransaction {
    pub metadata_transaction: ModifyMetadataTransaction,
    pub address: Address,
}

impl MetadataAddressTransaction {
    pub fn new(
        deadline: Deadline,
        address: Address,
        modifications: Vec<MetadataModification>,
        network_type: NetworkType,
    ) -> Result<Self> {
        ensure!(
            !modifications.is_empty(),
            errors_const::ERR_METADATA_EMPTY_MODIFICATIONS
        );

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            METADATA_ADDRESS_VERSION,
            TransactionType::ModifyMetadataAddress,
            network_type,
        );

        Ok(Self {
            metadata_transaction: ModifyMetadataTransaction {
                abs_transaction: abs_tx,
                metadata_type: MetadataType::MetadataAddressType,
                modifications,
            },
            address,
        })
    }
}

impl AbsTransaction for MetadataAddressTransaction {
    fn abs_transaction(&self) -> AbstractTransaction {
        self.metadata_transaction.abs_transaction()
    }
}

impl Transaction for MetadataAddressTransaction {
    fn size(&self) -> usize {
        self.metadata_transaction.size() + ADDRESS_SIZE
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

        self.metadata_transaction
            .embedded_to_bytes(&mut builder, address_vector, ADDRESS_SIZE)
    }

    fn set_aggregate(&mut self, signer: PublicAccount) {
        self.metadata_transaction.set_aggregate(signer)
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

impl fmt::Display for MetadataAddressTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
