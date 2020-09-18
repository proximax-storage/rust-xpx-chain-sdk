/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    ::std::fmt::{self, Display, Formatter},
    failure::_core::any::Any,
    serde_json::Value,
};

use crate::{
    account::{
        Account, AccountPropertiesEntityTypeModification, AccountPropertyType, PublicAccount,
    },
    models::consts::{
        ACCOUNT_PROPERTIES_ENTITY_MODIFICATION_SIZE, ACCOUNT_PROPERTY_ENTITY_TYPE_HEADER,
    },
    network::NetworkType,
    transaction::ACCOUNT_PROPERTY_ENTITY_TYPE_VERSION,
    Result,
};

use super::{
    buffer::account_properties as buffer, internal::sign_transaction,
    schema::account_property_transaction_schema, AbsTransaction, AbstractTransaction, Deadline,
    HashValue, SignedTransaction, Transaction, TransactionType,
};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountPropertiesEntityTypeTransaction {
    pub abs_transaction: AbstractTransaction,
    pub property_type: AccountPropertyType,
    pub modifications: Vec<AccountPropertiesEntityTypeModification>,
}

impl AccountPropertiesEntityTypeTransaction {
    pub fn new(
        deadline: Deadline,
        property_type: AccountPropertyType,
        modifications: Vec<AccountPropertiesEntityTypeModification>,
        network_type: NetworkType,
    ) -> crate::Result<Self> {
        ensure!(
            !modifications.is_empty(),
            "modifications must not be empty."
        );

        if property_type.value() & AccountPropertyType::AllowTransaction.value() == 0 {
            bail!("wrong propertyType for transaction account properties")
        }

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            ACCOUNT_PROPERTY_ENTITY_TYPE_VERSION,
            TransactionType::AccountRestrictionEntity,
            network_type,
        );

        Ok(Self {
            abs_transaction: abs_tx,
            property_type,
            modifications,
        })
    }
}

impl AbsTransaction for AccountPropertiesEntityTypeTransaction {
    fn abs_transaction(&self) -> AbstractTransaction {
        self.abs_transaction.to_owned()
    }
}

impl Transaction for AccountPropertiesEntityTypeTransaction {
    fn size(&self) -> usize {
        ACCOUNT_PROPERTY_ENTITY_TYPE_HEADER
            + (ACCOUNT_PROPERTIES_ENTITY_MODIFICATION_SIZE * self.modifications.len())
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
        let mut builder = fb::FlatBufferBuilder::new();

        let ml = self.modifications.len();

        let mut modifications_buffer: Vec<fb::WIPOffset<buffer::PropertyModificationBuffer<'a>>> =
            Vec::with_capacity(ml);

        for modification in self.modifications.iter() {
            let b_entity = modification.transaction_type.to_bytes();

            let v_entity = builder.create_vector(&b_entity);

            let mut modification_buffer =
                buffer::PropertyModificationBufferBuilder::new(&mut builder);
            modification_buffer.add_modification_type(modification.modification_type.value());
            modification_buffer.add_value(v_entity);

            modifications_buffer.push(modification_buffer.finish());
        }

        let v_modifications = builder.create_vector(&modifications_buffer);

        let abs_vector = self.abs_transaction.build_vector(&mut builder);

        let mut txn_builder = buffer::AccountPropertiesTransactionBufferBuilder::new(&mut builder);

        txn_builder.add_size_(self.size() as u32);
        txn_builder.add_signature(abs_vector.signature_vec);
        txn_builder.add_signer(abs_vector.signer_vec);
        txn_builder.add_version(abs_vector.version_vec);
        txn_builder.add_type_(abs_vector.type_vec);
        txn_builder.add_max_fee(abs_vector.max_fee_vec);
        txn_builder.add_deadline(abs_vector.deadline_vec);
        txn_builder.add_property_type(self.property_type.value());
        txn_builder.add_modification_count(self.modifications.len() as u8);
        txn_builder.add_modifications(v_modifications);

        let t = txn_builder.finish();

        builder.finish(t, None);

        let buf = builder.finished_data();

        Ok(account_property_transaction_schema().serialize(&mut buf.to_vec()))
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

impl Display for AccountPropertiesEntityTypeTransaction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
