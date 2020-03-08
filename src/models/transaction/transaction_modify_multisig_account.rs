use std::any::Any;

use serde_json::Value;

use crate::models::account::Account;
use crate::models::consts::{KEY_SIZE, MODIFY_MULTISIG_HEADER_SIZE};
use crate::models::multisig::CosignatoryModification;
use crate::models::network::NetworkType;
use crate::models::transaction::{AbstractTransaction, cosignatory_modification_array_to_buffer, Deadline, EntityTypeEnum, MODIFY_MULTISIG_VERSION, SignedTransaction, Transaction};
use crate::models::transaction::buffer::modify_multisig_account::buffers;
use crate::transaction::sign_transaction;

use super::schema::transfer_transaction_schema;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyMultisigAccountTransaction {
    pub abs_transaction: AbstractTransaction,
    pub min_removal_delta: i8,
    pub min_approval_delta: i8,
    pub modifications: Vec<CosignatoryModification>
}

impl ModifyMultisigAccountTransaction {
    pub fn new(deadline: Deadline,
               min_approval_delta: i8,
               min_removal_delta: i8,
               modifications: Vec<CosignatoryModification>,
               network_type: NetworkType
    ) -> crate::Result<Self> {
        ensure!(
        modifications.len() != 0 &&
        min_approval_delta != 0 &&
        min_removal_delta != 0,
        ERR_EMPTY_MODIFICATIONS
        );

        let abs_tx = AbstractTransaction {
            transaction_info: None,
            network_type,
            signature: "".to_string(),
            signer: Default::default(),
            version: MODIFY_MULTISIG_VERSION,
            transaction_type: EntityTypeEnum::ModifyMultisigAccount,
            max_fee: Default::default(),
            deadline,
        };

        Ok(ModifyMultisigAccountTransaction {
            abs_transaction: abs_tx,
            min_removal_delta,
            min_approval_delta,
            modifications
        })
    }
}

impl Transaction for ModifyMultisigAccountTransaction {
    fn transaction_hash(&self) -> &str {
        self.abs_transaction.get_hash()
    }

    fn abs_transaction(&self) -> AbstractTransaction {
        self.abs_transaction.to_owned()
    }

    fn size(&self) -> usize {
        MODIFY_MULTISIG_HEADER_SIZE + ((KEY_SIZE + 1) * self.modifications.len())
    }

    fn generate_bytes(&self) -> Vec<u8> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut _builder = fb::FlatBufferBuilder::new();

        let modification_vector = cosignatory_modification_array_to_buffer(
            builder, tx.Modifications);

        let abs_vector = &self.abs_transaction.generate_vector(&mut _builder);

        let mut txn_builder =
            buffers::ModifyMultisigAccountTransactionBufferBuilder::new(&mut _builder);

        txn_builder.add_size_(self.size() as u32);
        txn_builder.add_signature(fb::WIPOffset::new(*abs_vector.get("signatureV").unwrap()));
        txn_builder.add_signer(fb::WIPOffset::new(*abs_vector.get("signerV").unwrap()));
        txn_builder.add_version(*abs_vector.get("versionV").unwrap());
        txn_builder.add_type_(self.abs_transaction.transaction_type.get_value());
        txn_builder.add_max_fee(fb::WIPOffset::new(*abs_vector.get("feeV").unwrap()));
        txn_builder.add_deadline(fb::WIPOffset::new(*abs_vector.get("deadlineV").unwrap()));
        txn_builder.add_min_removal_delta(self.min_removal_delta);
        txn_builder.add_min_approval_delta(self.min_approval_delta);
        txn_builder.add_num_modifications(self.modifications.len() as u8);
        txn_builder.add_modifications(fb::WIPOffset::new(*modification_vector));

        let t = txn_builder.finish();
        _builder.finish(t, None);

        let buf = _builder.finished_data();
        modify_multisig_account_transaction().serialize(&mut Vec::from(buf))
    }

    fn generate_embedded_bytes(&self) -> Vec<u8> {
        unimplemented!()
    }

    fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    fn has_missing_signatures(&self) -> bool {
        unimplemented!()
    }

    fn sign_transaction_with(&self, account: Account, generation_hash: String)
                             -> crate::Result<SignedTransaction> {
        sign_transaction(self as &dyn Transaction, account, generation_hash)
    }

    fn entity_type(&self) -> EntityTypeEnum {
        self.abs_transaction.transaction_type.to_owned()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}