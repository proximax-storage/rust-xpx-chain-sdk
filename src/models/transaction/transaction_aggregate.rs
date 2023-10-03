/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use anyhow::Result;

use ::std::fmt;
use std::any::Any;
use serde_json::Value;

use crate::{
    account::Account,
    errors_const::ERR_EMPTY_INNER_TRANSACTION,
    helpers::TransactionHash,
    multisig::Cosignature,
    network::NetworkType,
};
use crate::account::PublicAccount;
use crate::errors_const::ERR_EMPTY_TRANSACTION_SIGNER;
use crate::models::consts::{AGGREGATE_BONDED_HEADER, DEAD_LINE_SIZE, MAX_FEE_SIZE, SIGNATURE_SIZE};
use crate::transaction::buffers;
use crate::transaction::internal::to_aggregate_transaction_bytes;
use crate::transaction::schema::aggregate_transaction_schema;

use super::{
    CommonTransaction,
    Deadline, internal::sign_transaction_with_cosignatures, SignedTransaction, Transaction, Transactions,
    TransactionType, TransactionVersion,
};

/// AggregateTransaction:
/// Transaction that combines multiple transactions together.
#[derive(Clone, Debug, Serialize, Builder, Deserialize)]
#[builder(
create_empty = "empty",
build_fn(validate = "Self::validate", error = "crate::api::error::Error")
)]
#[serde(rename_all = "camelCase")]
pub struct AggregateTransaction {
    /// Represents common transaction information..
    #[builder(private, pattern = "mutable")]
    pub common: CommonTransaction,
    /// An array of transaction cosignatures.
    #[builder(default)]
    pub cosignatures: Vec<Cosignature>,
    /// The array of transactions initiated by different accounts.
    pub inner_transactions: Transactions,
}

impl AggregateTransactionBuilder {
    fn validate(&self) -> Result<()> {
        if let Some(ref inner_transactions) = self.inner_transactions {
            ensure!(!inner_transactions.is_empty(), ERR_EMPTY_INNER_TRANSACTION);
        }

        Ok(())
    }

    /// The deadline method sets the deadline field.
    pub fn deadline(&mut self, value: Deadline) -> &mut AggregateTransactionBuilder {
        self.common.as_mut().map(|item| item.deadline = Some(value));
        self
    }

    /// The max_fee method sets the max_fee field.
    pub fn max_fee(&mut self, value: u64) -> &mut AggregateTransactionBuilder {
        self.common.as_mut().map(|item| item.max_fee = Some(value));
        self
    }
}

impl AggregateTransaction {
    pub fn builder_complete(network_type: NetworkType) -> AggregateTransactionBuilder {
        let common = CommonTransaction::create_from_type(
            TransactionType::AggregateComplete,
            network_type,
            TransactionVersion::AGGREGATE_COMPLETE,
            Some(Default::default()),
            None,
        );
        AggregateTransactionBuilder { common: Some(common), ..Default::default() }
    }

    pub fn builder_bonded(network_type: NetworkType) -> AggregateTransactionBuilder {
        let common = CommonTransaction::create_from_type(
            TransactionType::AggregateBonded,
            network_type,
            TransactionVersion::AGGREGATE_BONDED,
            Some(Default::default()),
            None,
        );
        AggregateTransactionBuilder { common: Some(common), ..Default::default() }
    }

    pub fn sign_with_cosignatories(
        self,
        account: Account,
        cosignatories: Vec<Account>,
        generation_hash: TransactionHash,
    ) -> Result<SignedTransaction> {
        sign_transaction_with_cosignatures(self, account, cosignatories, generation_hash)
    }
}

impl fmt::Display for AggregateTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}

#[typetag::serde]
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

    fn get_common_transaction(&self) -> CommonTransaction {
        self.common.to_owned()
    }

    fn to_serializer<'a>(&self) -> Vec<u8> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut _builder = fb::FlatBufferBuilder::new();

        let mut txsb: Vec<u8> = Vec::new();
        for tx in &self.inner_transactions {
            let mut tx_byte =
                to_aggregate_transaction_bytes(tx).expect(ERR_EMPTY_TRANSACTION_SIGNER);
            txsb.append(&mut tx_byte)
        }

        let tx_vec = _builder.create_vector(&txsb);

        let abs_vector = self.common.build_vector(&mut _builder);

        let mut txn_builder = buffers::AggregateTransactionBufferBuilder::new(&mut _builder);

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

        aggregate_transaction_schema().serialize(&mut buf.to_vec())
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
