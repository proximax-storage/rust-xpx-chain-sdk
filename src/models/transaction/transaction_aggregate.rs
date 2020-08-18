/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {::std::fmt, failure::_core::any::Any, serde_json::Value};

use crate::{
    models::{
        account::{Account, PublicAccount},
        consts::{AGGREGATE_BONDED_HEADER, DEAD_LINE_SIZE, MAX_FEE_SIZE, SIGNATURE_SIZE},
        errors_const::ERR_EMPTY_INNER_TRANSACTION,
        multisig::Cosignature,
        network::NetworkType,
    },
    Result,
};

use super::{
    buffer::aggregate as buffer,
    internal::{
        sign_transaction, sign_transaction_with_cosignatures, to_aggregate_transaction_bytes,
    },
    schema::aggregate_transaction_schema,
    AbsTransaction, AbstractTransaction, Deadline, HashValue, SignedTransaction, Transaction,
    TransactionType, Transactions, AGGREGATE_BONDED_VERSION, AGGREGATE_COMPLETED_VERSION,
};

/// AggregateTransaction:
/// Transaction that combines multiple transactions together.
#[derive(Clone, Debug, Serialize)]
pub struct AggregateTransaction {
    pub abs_transaction: AbstractTransaction,
    /// An array of transaction cosignatures.
    pub cosignatures: Vec<Cosignature>,
    /// The array of transactions initiated by different accounts.
    pub inner_transactions: Transactions,
}

impl AggregateTransaction {
    pub fn new_complete(
        deadline: Deadline,
        inner_txs: Vec<Box<dyn Transaction>>,
        network_type: NetworkType,
    ) -> Result<AggregateTransaction> {
        ensure!(!inner_txs.is_empty(), ERR_EMPTY_INNER_TRANSACTION);

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            AGGREGATE_COMPLETED_VERSION,
            TransactionType::AggregateComplete,
            network_type,
        );

        Ok(Self {
            abs_transaction: abs_tx,
            cosignatures: vec![],
            inner_transactions: inner_txs,
        })
    }

    pub fn new_bonded(
        deadline: Deadline,
        inner_txs: Vec<Box<dyn Transaction>>,
        network_type: NetworkType,
    ) -> Result<Self> {
        ensure!(!inner_txs.is_empty(), ERR_EMPTY_INNER_TRANSACTION);

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            AGGREGATE_BONDED_VERSION,
            TransactionType::AggregateBonded,
            network_type,
        );

        Ok(Self {
            abs_transaction: abs_tx,
            cosignatures: vec![],
            inner_transactions: inner_txs,
        })
    }

    pub(crate) fn sign_with_cosignatories(
        self,
        account: Account,
        cosignatories: Vec<Account>,
        generation_hash: HashValue,
    ) -> crate::Result<SignedTransaction> {
        sign_transaction_with_cosignatures(self, account, cosignatories, generation_hash)
    }
}

impl fmt::Display for AggregateTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

impl AbsTransaction for AggregateTransaction {
    fn abs_transaction(&self) -> AbstractTransaction {
        self.abs_transaction.to_owned()
    }
}

impl Transaction for AggregateTransaction {
    fn size(&self) -> usize {
        let mut size_of_inner_transactions = 0;
        self.inner_transactions.iter().for_each(|itx| {
            size_of_inner_transactions +=
                itx.size() - SIGNATURE_SIZE - MAX_FEE_SIZE - DEAD_LINE_SIZE
        });
        AGGREGATE_BONDED_HEADER + size_of_inner_transactions
    }

    fn as_value(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    fn sign_transaction_with(
        self,
        account: Account,
        generation_hash: HashValue,
    ) -> crate::Result<SignedTransaction> {
        sign_transaction(self, account, generation_hash)
    }

    fn embedded_to_bytes(&self) -> Result<Vec<u8>> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut _builder = fb::FlatBufferBuilder::new();

        let mut txsb: Vec<u8> = Vec::new();
        for tx in &self.inner_transactions {
            let mut tx_byte = to_aggregate_transaction_bytes(tx)?;
            txsb.append(&mut tx_byte)
        }

        let tx_vec = _builder.create_vector_direct(&txsb);

        let abs_vector = self.abs_transaction.build_vector(&mut _builder);

        let mut txn_builder = buffer::AggregateTransactionBufferBuilder::new(&mut _builder);

        txn_builder.add_size_(self.size() as u32);
        txn_builder.add_signature(abs_vector.signature_vec);
        txn_builder.add_signer(abs_vector.signer_vec);
        txn_builder.add_version(abs_vector.version_vec);
        txn_builder.add_type_(abs_vector.type_vec);
        txn_builder.add_max_fee(abs_vector.max_fee_vec);
        txn_builder.add_deadline(abs_vector.deadline_vec);
        txn_builder.add_transactions_size(txsb.len() as u32);
        txn_builder.add_transactions(tx_vec);

        let t = txn_builder.finish();
        _builder.finish(t, None);

        let buf = _builder.finished_data();

        Ok(aggregate_transaction_schema().serialize(&mut buf.to_vec()))
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
