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
        account::{Account, PublicAccount},
        consts::NAMESPACE_SIZE,
        errors_const,
        metadata::{MetadataModification, MetadataType},
        namespace::NamespaceId,
        network::NetworkType,
    },
    AssetId, Result,
};

use super::{
    internal::sign_transaction, AbsTransaction, AbstractTransaction, Deadline,
    ModifyMetadataTransaction, SignedTransaction, Transaction, TransactionType,
    METADATA_NAMESPACE_VERSION,
};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataNamespaceTransaction {
    pub metadata_transaction: ModifyMetadataTransaction,
    pub namespace_id: NamespaceId,
}

impl MetadataNamespaceTransaction {
    pub fn new(
        deadline: Deadline,
        namespace_id: NamespaceId,
        modifications: Vec<MetadataModification>,
        network_type: NetworkType,
    ) -> Result<Self> {
        ensure!(
            !modifications.is_empty(),
            errors_const::ERR_METADATA_EMPTY_MODIFICATIONS
        );

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            METADATA_NAMESPACE_VERSION,
            TransactionType::ModifyMetadataNamespace,
            network_type,
        );

        Ok(Self {
            metadata_transaction: ModifyMetadataTransaction {
                abs_transaction: abs_tx,
                metadata_type: MetadataType::MetadataNamespaceType,
                modifications,
            },
            namespace_id,
        })
    }
}

impl AbsTransaction for MetadataNamespaceTransaction {
    fn abs_transaction(&self) -> AbstractTransaction {
        self.metadata_transaction.abs_transaction()
    }
}

impl Transaction for MetadataNamespaceTransaction {
    fn size(&self) -> usize {
        self.metadata_transaction.size() + NAMESPACE_SIZE
    }

    fn as_value(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    fn sign_transaction_with(
        self,
        account: Account,
        generation_hash: String,
    ) -> Result<SignedTransaction> {
        sign_transaction(self, account, generation_hash)
    }

    fn embedded_to_bytes(&self) -> Result<Vec<u8>> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut builder = fb::FlatBufferBuilder::new();

        let namespace_id_bytes = self.namespace_id.to_bytes();

        let namespace_id_vector = builder.create_vector_direct(&namespace_id_bytes);

        self.metadata_transaction.embedded_to_bytes(
            &mut builder,
            namespace_id_vector,
            NAMESPACE_SIZE,
        )
    }

    fn set_aggregate(&mut self, signer: PublicAccount) {
        self.metadata_transaction.set_aggregate(signer)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn box_clone(&self) -> Box<dyn Transaction + 'static> {
        Box::new((*self).clone())
    }
}

impl fmt::Display for MetadataNamespaceTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
