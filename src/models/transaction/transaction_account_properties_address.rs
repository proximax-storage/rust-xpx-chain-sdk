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
    account::{Account, AccountPropertiesAddressModification, AccountPropertyType, PublicAccount},
    models::consts::{
        ACCOUNT_PROPERTIES_ADDRESS_MODIFICATION_SIZE, ACCOUNT_PROPERTY_ADDRESS_HEADER,
    },
    network::NetworkType,
    Result,
};

use super::{
    AbstractTransaction, AbsTransaction,
    ACCOUNT_PROPERTY_ADDRESS_VERSION, buffer::account_properties as buffer, Deadline, HashValue,
    internal::sign_transaction, schema::account_property_transaction_schema, SignedTransaction, Transaction, TransactionType,
};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountPropertiesAddressTransaction {
    pub abs_transaction: AbstractTransaction,
    pub property_type: AccountPropertyType,
    pub modifications: Vec<AccountPropertiesAddressModification>,
}

impl AccountPropertiesAddressTransaction {
    pub fn new(
        deadline: Deadline,
        property_type: AccountPropertyType,
        modifications: Vec<AccountPropertiesAddressModification>,
        network_type: NetworkType,
    ) -> crate::Result<Self> {
        ensure!(
            !modifications.is_empty(),
            "modifications must not be empty."
        );

        if property_type.value() & AccountPropertyType::AllowAddress.value() == 0 {
            bail!("wrong propertyType for address account properties")
        }

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            ACCOUNT_PROPERTY_ADDRESS_VERSION,
            TransactionType::AccountRestrictionAddress,
            network_type,
        );

        Ok(Self {
            abs_transaction: abs_tx,
            property_type,
            modifications,
        })
    }
}

impl AbsTransaction for AccountPropertiesAddressTransaction {
    fn abs_transaction(&self) -> AbstractTransaction {
        self.abs_transaction.to_owned()
    }
}

impl Transaction for AccountPropertiesAddressTransaction {
    fn size(&self) -> usize {
        ACCOUNT_PROPERTY_ADDRESS_HEADER
            + (ACCOUNT_PROPERTIES_ADDRESS_MODIFICATION_SIZE * self.modifications.len())
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
            let address = modification.address.as_bytes();

            let v_address = builder.create_vector(address);

            let mut modification_buffer =
                buffer::PropertyModificationBufferBuilder::new(&mut builder);
            modification_buffer.add_modification_type(modification.modification_type.value());
            modification_buffer.add_value(v_address);

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

impl Display for AccountPropertiesAddressTransaction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
