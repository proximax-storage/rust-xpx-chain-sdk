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
        consts::MOSAIC_ID_SIZE,
        errors_const,
        metadata::{MetadataModification, MetadataType},
        mosaic::MosaicId,
        network::NetworkType,
    },
    AssetId, Result,
};

use super::{
    internal::sign_transaction, AbsTransaction, AbstractTransaction, Deadline, HashValue,
    ModifyMetadataTransaction, SignedTransaction, Transaction, TransactionType,
    METADATA_MOSAIC_VERSION,
};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataMosaicTransaction {
    pub metadata_transaction: ModifyMetadataTransaction,
    pub mosaic_id: MosaicId,
}

impl MetadataMosaicTransaction {
    pub fn new(
        deadline: Deadline,
        mosaic_id: MosaicId,
        modifications: Vec<MetadataModification>,
        network_type: NetworkType,
    ) -> Result<Self> {
        ensure!(
            !modifications.is_empty(),
            errors_const::ERR_METADATA_EMPTY_MODIFICATIONS
        );

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            METADATA_MOSAIC_VERSION,
            TransactionType::ModifyMetadataMosaic,
            network_type,
        );

        Ok(Self {
            metadata_transaction: ModifyMetadataTransaction {
                abs_transaction: abs_tx,
                metadata_type: MetadataType::MetadataMosaicType,
                modifications,
            },
            mosaic_id,
        })
    }
}

impl AbsTransaction for MetadataMosaicTransaction {
    fn abs_transaction(&self) -> AbstractTransaction {
        self.metadata_transaction.abs_transaction()
    }
}

impl Transaction for MetadataMosaicTransaction {
    fn size(&self) -> usize {
        self.metadata_transaction.size() + MOSAIC_ID_SIZE
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

        let mosaic_id_bytes = self.mosaic_id.to_bytes();

        let mosaic_id_vector = builder.create_vector_direct(&mosaic_id_bytes);

        self.metadata_transaction
            .embedded_to_bytes(&mut builder, mosaic_id_vector, MOSAIC_ID_SIZE)
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

impl fmt::Display for MetadataMosaicTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
