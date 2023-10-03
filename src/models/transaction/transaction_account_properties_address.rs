/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use anyhow::{bail, ensure, Result};

use ::std::fmt::{self, Display, Formatter};
use std::any::Any;
use serde_json::Value;

use crate::{
    account::{AccountPropertiesAddressModification, AccountPropertyType},
    network::NetworkType,
};
use crate::account::PublicAccount;
use crate::models::consts::{ACCOUNT_PROPERTIES_ADDRESS_MODIFICATION_SIZE, ACCOUNT_PROPERTY_ADDRESS_HEADER};
use crate::transaction::buffers::{AccountPropertiesTransactionBufferBuilder, PropertyModificationBuffer, PropertyModificationBufferBuilder};
use crate::transaction::schema::account_property_transaction_schema;

use super::{
    CommonTransaction, Deadline, Transaction, TransactionType, TransactionVersion,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountPropertiesAddressTransaction {
    pub common: CommonTransaction,
    pub property_type: AccountPropertyType,
    pub modifications: Vec<AccountPropertiesAddressModification>,
}

impl AccountPropertiesAddressTransaction {
    pub fn create(
        deadline: Deadline,
        property_type: AccountPropertyType,
        modifications: Vec<AccountPropertiesAddressModification>,
        network_type: NetworkType,
        max_fee: Option<u64>,
    ) -> Result<Self> {
        ensure!(!modifications.is_empty(), "modifications must not be empty.");

        if property_type.value() & AccountPropertyType::AllowAddress.value() == 0 {
            bail!("wrong propertyType for address account properties")
        }

        let abs_tx = CommonTransaction::create_from_type(
            TransactionType::AccountRestrictionAddress,
            network_type,
            TransactionVersion::MODIFY_ACCOUNT_RESTRICTION_ADDRESS,
            Some(deadline),
            max_fee,
        );

        Ok(Self { common: abs_tx, property_type, modifications })
    }
}

#[typetag::serde]
impl Transaction for AccountPropertiesAddressTransaction {
    fn size(&self) -> usize {
        ACCOUNT_PROPERTY_ADDRESS_HEADER
            + (ACCOUNT_PROPERTIES_ADDRESS_MODIFICATION_SIZE * self.modifications.len())
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
        let mut builder = fb::FlatBufferBuilder::new();

        let ml = self.modifications.len();

        let mut modifications_buffer: Vec<fb::WIPOffset<PropertyModificationBuffer<'a>>> =
            Vec::with_capacity(ml);

        for modification in self.modifications.iter() {
            let address = modification.address.as_bytes();

            let v_address = builder.create_vector(address);

            let mut modification_buffer = PropertyModificationBufferBuilder::new(&mut builder);
            modification_buffer.add_modification_type(modification.modification_type.value());
            modification_buffer.add_value(v_address);

            modifications_buffer.push(modification_buffer.finish());
        }

        let v_modifications = builder.create_vector(&modifications_buffer);

        let abs_vector = self.common.build_vector(&mut builder);

        let mut txn_builder = AccountPropertiesTransactionBufferBuilder::new(&mut builder);

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

        account_property_transaction_schema().serialize(&mut buf.to_vec())
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

impl Display for AccountPropertiesAddressTransaction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}
