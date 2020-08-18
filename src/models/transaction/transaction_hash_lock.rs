/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {failure::_core::any::Any, serde_json::Value, std::fmt};

use crate::{
    models::{
        account::{Account, PublicAccount},
        consts::LOCK_SIZE,
        mosaic::Mosaic,
        network::NetworkType,
        uint_64::Uint64,
    },
    Result,
};

use super::{
    buffer::lock_funds as buffer, internal::sign_transaction,
    schema::lock_funds_transaction_schema, AbsTransaction, AbstractTransaction, Deadline,
    HashValue, SignedTransaction, Transaction, TransactionType, LOCK_VERSION,
};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LockFundsTransaction {
    pub abs_transaction: AbstractTransaction,
    pub mosaic: Mosaic,
    pub duration: Uint64,
    pub signed_transaction: SignedTransaction,
}

impl LockFundsTransaction {
    pub fn new(
        deadline: Deadline,
        mosaic: Mosaic,
        duration: Uint64,
        signed_tx: SignedTransaction,
        network_type: NetworkType,
    ) -> Result<Self> {
        ensure!(
            signed_tx.get_type() == TransactionType::AggregateBonded,
            "signed_tx must be of type AggregateBonded."
        );

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            LOCK_VERSION,
            TransactionType::Lock,
            network_type,
        );

        Ok(Self {
            abs_transaction: abs_tx,
            mosaic,
            duration,
            signed_transaction: signed_tx,
        })
    }
}

impl AbsTransaction for LockFundsTransaction {
    fn abs_transaction(&self) -> AbstractTransaction {
        self.abs_transaction.to_owned()
    }
}

impl Transaction for LockFundsTransaction {
    fn size(&self) -> usize {
        LOCK_SIZE
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

    fn embedded_to_bytes<'a>(&self) -> Result<Vec<u8>> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut _builder = fb::FlatBufferBuilder::new();

        let mosaic_id_vector = _builder.create_vector_direct(&self.mosaic.asset_id.to_u32_array());
        let amount_vector = _builder.create_vector_direct(&self.mosaic.amount.to_i32_array());
        let duration_vector = _builder.create_vector_direct(&self.duration.to_i32_array());
        let hash_vector = _builder.create_vector_direct(&self.signed_transaction.hash_to_bytes());

        let abs_vector = self.abs_transaction.build_vector(&mut _builder);

        let mut txn_builder = buffer::LockFundsTransactionBufferBuilder::new(&mut _builder);

        txn_builder.add_size_(self.size() as u32);
        txn_builder.add_signature(abs_vector.signature_vec);
        txn_builder.add_signer(abs_vector.signer_vec);
        txn_builder.add_version(abs_vector.version_vec);
        txn_builder.add_type_(abs_vector.type_vec);
        txn_builder.add_max_fee(abs_vector.max_fee_vec);
        txn_builder.add_deadline(abs_vector.deadline_vec);
        txn_builder.add_mosaic_id(mosaic_id_vector);
        txn_builder.add_mosaic_amount(amount_vector);
        txn_builder.add_duration(duration_vector);
        txn_builder.add_hash(hash_vector);

        let t = txn_builder.finish();
        _builder.finish(t, None);

        let buf = _builder.finished_data();

        Ok(lock_funds_transaction_schema().serialize(&mut buf.to_vec()))
    }

    fn set_aggregate(&mut self, signer: PublicAccount) {
        self.abs_transaction.set_aggregate(signer)
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

impl fmt::Display for LockFundsTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
