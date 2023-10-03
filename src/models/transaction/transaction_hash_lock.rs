/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::any::Any;
use anyhow::Result;

use std::fmt;
use serde_json::Value;

use crate::{AsUint64, mosaic::Mosaic, network::NetworkType};
use crate::account::PublicAccount;
use crate::models::consts::LOCK_SIZE;
use crate::transaction::buffers;
use crate::transaction::schema::lock_funds_transaction_schema;

use super::{
    CommonTransaction, Deadline, SignedTransaction,
    Transaction, TransactionType, TransactionVersion,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LockFundsTransaction {
    pub common: CommonTransaction,
    pub mosaic: Mosaic,
    pub duration: u64,
    pub signed_transaction: SignedTransaction,
}

impl LockFundsTransaction {
    pub fn create(
        deadline: Deadline,
        mosaic: Mosaic,
        duration: u64,
        signed_tx: SignedTransaction,
        network_type: NetworkType,
        max_fee: Option<u64>,
    ) -> Result<Self> {
        ensure!(
			signed_tx.get_type() == TransactionType::AggregateBonded,
			"signed_tx must be of type AggregateBonded."
		);

        let abs_tx = CommonTransaction::create_from_type(
            TransactionType::HashLock,
            network_type,
            TransactionVersion::HASH_LOCK,
            Some(deadline),
            max_fee,
        );

        Ok(Self { common: abs_tx, mosaic, duration, signed_transaction: signed_tx })
    }
}

#[typetag::serde]
impl Transaction for LockFundsTransaction {
    fn size(&self) -> usize {
        LOCK_SIZE
    }

    fn as_value(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    fn get_common_transaction(&self) -> CommonTransaction {
        self.common.to_owned()
    }

    fn to_serializer<'a>(&self) -> Vec<u8> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut _builder = fb::FlatBufferBuilder::new();

        let mosaic_id_vector = _builder.create_vector(&self.mosaic.asset_id.to_dto());
        let amount_vector = _builder.create_vector(&self.mosaic.amount.to_dto());
        let duration_vector = _builder.create_vector(&self.duration.to_dto());
        let hash_vector = _builder.create_vector(self.signed_transaction.hash_to_bytes());

        let abs_vector = self.common.build_vector(&mut _builder);

        let mut txn_builder = buffers::LockFundsTransactionBufferBuilder::new(&mut _builder);

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

        lock_funds_transaction_schema().serialize(&mut buf.to_vec())
    }

    fn set_aggregate(&mut self, signer: PublicAccount) {
        self.common.set_aggregate(signer)
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
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}
