/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::any::Any;
use anyhow::Result;

use std::fmt;
use serde_json::Value;

use crate::{
    account::Address,
    errors_const,
    metadata::{MetadataModification, MetadataType},
    network::NetworkType,
};
use crate::account::PublicAccount;
use crate::models::consts::ADDRESS_SIZE;

use super::{
    CommonTransaction, Deadline, ModifyMetadataTransaction, Transaction, TransactionType,
    TransactionVersion,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataAddressTransaction {
    pub metadata_transaction: ModifyMetadataTransaction,
    pub address: Address,
}

impl MetadataAddressTransaction {
    pub fn create(
        deadline: Deadline,
        address: Address,
        modifications: Vec<MetadataModification>,
        network_type: NetworkType,
        max_fee: Option<u64>,
    ) -> Result<Self> {
        ensure!(!modifications.is_empty(), errors_const::ERR_METADATA_EMPTY_MODIFICATIONS);

        let common = CommonTransaction::create_from_type(
            TransactionType::ModifyMetadataAddress,
            network_type,
            TransactionVersion::ACCOUNT_METADATA,
            Some(deadline),
            max_fee,
        );

        Ok(Self {
            metadata_transaction: ModifyMetadataTransaction {
                common,
                metadata_type: MetadataType::MetadataAddressType,
                modifications,
            },
            address,
        })
    }
}

#[typetag::serde]
impl Transaction for MetadataAddressTransaction {
    fn size(&self) -> usize {
        self.metadata_transaction.size() + ADDRESS_SIZE
    }

    fn as_value(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    fn get_common_transaction(&self) -> CommonTransaction {
        self.metadata_transaction.common()
    }

    fn to_serializer<'a>(&self) -> Vec<u8> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut builder = fb::FlatBufferBuilder::new();

        let address_bytes = self.address.as_bytes();

        let address_vector = builder.create_vector(address_bytes);

        self.metadata_transaction
            .embedded_to_bytes(&mut builder, address_vector, ADDRESS_SIZE)
    }

    fn set_aggregate(&mut self, signer: PublicAccount) {
        self.metadata_transaction.set_aggregate(signer)
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

impl fmt::Display for MetadataAddressTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}
