/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::any::Any;
use std::fmt;

use anyhow::Result;
use serde_json::Value;

use crate::{
    errors_const,
    multisig::CosignatoryModification,
    network::NetworkType,
};
use crate::account::PublicAccount;
use crate::models::consts::{KEY_SIZE, MODIFY_MULTISIG_HEADER_SIZE};
use crate::transaction::buffers;
use crate::transaction::internal::cosignatory_modification_array_to_buffer;
use crate::transaction::schema::modify_multisig_account_transaction_schema;

use super::{
    CommonTransaction, Deadline, Transaction,
    TransactionType, TransactionVersion,
};

#[derive(Clone, Debug, Serialize, Builder, Deserialize)]
#[builder(
create_empty = "empty",
build_fn(validate = "Self::validate", error = "crate::api::error::Error")
)]
#[serde(rename_all = "camelCase")]
pub struct ModifyMultisigAccountTransaction {
    /// Represents common transaction information..
    #[builder(private, pattern = "mutable")]
    pub common: CommonTransaction,
    pub min_removal_delta: i8,
    pub min_approval_delta: i8,
    pub modifications: Vec<CosignatoryModification>,
}

impl ModifyMultisigAccountTransactionBuilder {
    fn validate(&self) -> Result<()> {
        if let Some(ref modifications) = self.modifications {
            ensure!(!modifications.is_empty(), errors_const::ERR_EMPTY_MODIFICATIONS);
        }

        if let Some(min_approval_delta) = self.min_approval_delta {
            ensure!(min_approval_delta != 0, errors_const::ERR_EMPTY_MODIFICATIONS);
        }

        if let Some(min_removal_delta) = self.min_removal_delta {
            ensure!(min_removal_delta != 0, errors_const::ERR_EMPTY_MODIFICATIONS);
        }

        Ok(())
    }

    /// The deadline method sets the deadline field.
    pub fn deadline(&mut self, value: Deadline) -> &mut ModifyMultisigAccountTransactionBuilder {
        self.common.as_mut().map(|item| item.deadline = Some(value));
        self
    }

    /// The max_fee method sets the max_fee field.
    pub fn max_fee(&mut self, value: u64) -> &mut ModifyMultisigAccountTransactionBuilder {
        self.common.as_mut().map(|item| item.max_fee = Some(value));
        self
    }
}

impl ModifyMultisigAccountTransaction {
    /// Build a transfer transaction object.
    pub fn builder(network_type: NetworkType) -> ModifyMultisigAccountTransactionBuilder {
        let common = CommonTransaction::create_from_type(
            TransactionType::MultisigAccountModify,
            network_type,
            TransactionVersion::MULTISIG_ACCOUNT_MODIFICATION,
            Some(Default::default()),
            None,
        );
        ModifyMultisigAccountTransactionBuilder { common: Some(common), ..Default::default() }
    }
}

#[typetag::serde]
impl Transaction for ModifyMultisigAccountTransaction {
    fn size(&self) -> usize {
        MODIFY_MULTISIG_HEADER_SIZE + ((KEY_SIZE + 1) * self.modifications.len())
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

        let modifications = self.clone().modifications;

        let modification_vector =
            cosignatory_modification_array_to_buffer(&mut _builder, modifications);

        let abs_vector = self.common.build_vector(&mut _builder);

        let mut txn_builder =
            buffers::ModifyMultisigAccountTransactionBufferBuilder::new(&mut _builder);
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

        modify_multisig_account_transaction_schema().serialize(&mut buf.to_vec())
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

impl fmt::Display for ModifyMultisigAccountTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}
