/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::any::Any;
use std::fmt;
use serde_json::Value;

use crate::{
    account::Address,
    alias::AliasActionType,
    namespace::NamespaceId,
    network::NetworkType,
};
use crate::account::PublicAccount;
use crate::models::consts::ADDRESS_SIZE;

use super::{
    AliasTransaction, CommonTransaction, Deadline, Transaction, TransactionType, TransactionVersion,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressAliasTransaction {
    pub alias_transaction: AliasTransaction,
    pub address: Address,
}

impl AddressAliasTransaction {
    pub fn create(
        deadline: Deadline,
        address: Address,
        namespace_id: NamespaceId,
        action_type: AliasActionType,
        network_type: NetworkType,
        max_fee: Option<u64>,
    ) -> Self {
        let abs_tx = CommonTransaction::create_from_type(
            TransactionType::AddressAlias,
            network_type,
            TransactionVersion::ADDRESS_ALIAS,
            Some(deadline),
            max_fee,
        );

        Self {
            alias_transaction: AliasTransaction { common: abs_tx, action_type, namespace_id },
            address,
        }
    }
}

#[typetag::serde]
impl Transaction for AddressAliasTransaction {
    fn size(&self) -> usize {
        self.alias_transaction.size() + ADDRESS_SIZE
    }

    fn as_value(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    fn get_common_transaction(&self) -> CommonTransaction {
        self.alias_transaction.common()
    }

    fn to_serializer<'a>(&self) -> Vec<u8> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut builder = fb::FlatBufferBuilder::new();

        let address_bytes = self.address.as_bytes();

        let address_vector = builder.create_vector(address_bytes);

        self.alias_transaction.to_serializer(&mut builder, address_vector, ADDRESS_SIZE)
    }

    fn set_aggregate(&mut self, signer: PublicAccount) {
        self.alias_transaction.set_aggregate(signer)
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

impl fmt::Display for AddressAliasTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}
