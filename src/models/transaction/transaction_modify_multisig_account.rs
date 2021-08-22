/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {::std::any::Any, serde_json::Value};

use crate::{
    models::{
        account::{Account, PublicAccount},
        consts::{KEY_SIZE, MODIFY_MULTISIG_HEADER_SIZE},
        errors_const::ERR_EMPTY_MODIFICATIONS,
        multisig::CosignatoryModification,
        network::NetworkType,
    },
    Result,
};

use super::{
    AbstractTransaction,
    AbsTransaction,
    buffer::modify_multisig_account as buffer,
    Deadline, HashValue, internal::{cosignatory_modification_array_to_buffer, sign_transaction}, MODIFY_MULTISIG_VERSION, schema::modify_multisig_account_transaction_schema, SignedTransaction,
    Transaction, TransactionType,
};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyMultisigAccountTransaction {
    pub abs_transaction: AbstractTransaction,
    pub min_removal_delta: i8,
    pub min_approval_delta: i8,
    pub modifications: Vec<CosignatoryModification>,
}

impl ModifyMultisigAccountTransaction {
    pub fn new(
        deadline: Deadline,
        min_approval_delta: i8,
        min_removal_delta: i8,
        modifications: Vec<CosignatoryModification>,
        network_type: NetworkType,
    ) -> Result<Self> {
        ensure!(
            !modifications.is_empty() && min_approval_delta != 0 && min_removal_delta != 0,
            ERR_EMPTY_MODIFICATIONS
        );

        let abs_tx = AbstractTransaction {
            transaction_info: None,
            network_type,
            signature: None,
            signer: Default::default(),
            version: MODIFY_MULTISIG_VERSION,
            transaction_type: TransactionType::ModifyMultisigAccount,
            max_fee: None,
            deadline: Some(deadline),
        };

        Ok(Self {
            abs_transaction: abs_tx,
            min_removal_delta,
            min_approval_delta,
            modifications,
        })
    }
}

impl AbsTransaction for ModifyMultisigAccountTransaction {
    fn abs_transaction(&self) -> AbstractTransaction {
        self.abs_transaction.to_owned()
    }
}

impl Transaction for ModifyMultisigAccountTransaction {
    fn size(&self) -> usize {
        MODIFY_MULTISIG_HEADER_SIZE + ((KEY_SIZE + 1) * self.modifications.len())
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
        let mut _builder = fb::FlatBufferBuilder::new();

        let modifications = self.clone().modifications;

        let modification_vector =
            cosignatory_modification_array_to_buffer(&mut _builder, modifications);

        let abs_vector = self.abs_transaction.build_vector(&mut _builder);

        let mut txn_builder =
            buffer::ModifyMultisigAccountTransactionBufferBuilder::new(&mut _builder);
        txn_builder.add_size_(self.size() as u32);
        txn_builder.add_signature(abs_vector.signature_vec);
        txn_builder.add_signer(abs_vector.signer_vec);
        txn_builder.add_version(abs_vector.version_vec);
        txn_builder.add_type_(abs_vector.type_vec);
        txn_builder.add_max_fee(abs_vector.max_fee_vec);
        txn_builder.add_deadline(abs_vector.deadline_vec);

        txn_builder.add_min_removal_delta(self.min_removal_delta);
        txn_builder.add_min_approval_delta(self.min_approval_delta);
        txn_builder.add_num_modifications(self.modifications.len() as u8);
        txn_builder.add_modifications(fb::WIPOffset::new(modification_vector));

        let t = txn_builder.finish();
        _builder.finish(t, None);

        let buf = _builder.finished_data();

        Ok(modify_multisig_account_transaction_schema().serialize(&mut buf.to_vec()))
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

impl core::fmt::Display for ModifyMultisigAccountTransaction {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
