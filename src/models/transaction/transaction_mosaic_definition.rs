/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use ::std::fmt;
use std::any::Any;
use serde_json::Value;

use crate::{
    account::PublicAccount,
    mosaic::{MosaicId, MosaicNonce, MosaicProperties},
    network::NetworkType,
};
use crate::models::consts::{MOSAIC_DEFINITION_TRANSACTION_HEADER_SIZE, MOSAIC_OPTIONAL_PROPERTY_SIZE};
use crate::transaction::buffers;
use crate::transaction::internal::mosaic_property_array_to_buffer;
use crate::transaction::schema::mosaic_definition_transaction_schema;

use super::{
    CommonTransaction,
    deadline::Deadline, Transaction, TransactionType,
    TransactionVersion,
};

#[derive(Clone, Debug, Serialize, Builder, Deserialize)]
#[builder(create_empty = "empty", build_fn(error = "crate::api::error::Error"))]
#[serde(rename_all = "camelCase")]
pub struct MosaicDefinitionTransaction {
    /// Represents common transaction information..
    #[builder(private, pattern = "mutable")]
    pub common: CommonTransaction,
    pub properties: MosaicProperties,
    pub mosaic_nonce: MosaicNonce,
    #[builder(setter(name = "owner_public_account", custom))]
    pub mosaic_id: MosaicId,
}

impl MosaicDefinitionTransactionBuilder {
    /// The deadline method sets the deadline field.
    pub fn deadline(&mut self, value: Deadline) -> &mut MosaicDefinitionTransactionBuilder {
        self.common.as_mut().map(|item| item.deadline = Some(value));
        self
    }

    /// The max_fee method sets the max_fee field.
    pub fn max_fee(&mut self, value: u64) -> &mut MosaicDefinitionTransactionBuilder {
        self.common.as_mut().map(|item| item.max_fee = Some(value));
        self
    }

    pub fn owner_public_account(
        &mut self,
        value: PublicAccount,
    ) -> &mut MosaicDefinitionTransactionBuilder {
        let nonce = self.mosaic_nonce.unwrap_or(MosaicNonce::random());
        let mosaic_id = MosaicId::create_from_nonce(nonce, value);
        self.mosaic_id = Some(mosaic_id);
        self
    }
}

impl MosaicDefinitionTransaction {
    /// Build a MosaicDefinitionTransaction transaction object.
    pub fn builder(network_type: NetworkType) -> MosaicDefinitionTransactionBuilder {
        let common = CommonTransaction::create_from_type(
            TransactionType::MosaicDefinition,
            network_type,
            TransactionVersion::MOSAIC_DEFINITION,
            Some(Default::default()),
            None,
        );
        MosaicDefinitionTransactionBuilder { common: Some(common), ..Default::default() }
    }
}

#[typetag::serde]
impl Transaction for MosaicDefinitionTransaction {
    fn size(&self) -> usize {
        MOSAIC_DEFINITION_TRANSACTION_HEADER_SIZE
            + self.properties.optional_properties.len() * MOSAIC_OPTIONAL_PROPERTY_SIZE
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

        let mut flags: u8 = 0;
        if self.properties.supply_mutable {
            flags += MosaicProperties::FLAG_SUPPLY_MUTABLE;
        }

        if self.properties.transferable {
            flags += MosaicProperties::FLAG_TRANSFERABLE;
        }

        let mosaic_vec = builder.create_vector(&self.mosaic_id.to_dto());
        let property_vec = mosaic_property_array_to_buffer(
            &mut builder,
            self.properties.clone().optional_properties,
        );

        let abs_vector = self.common.build_vector(&mut builder);

        let mut txn_builder = buffers::MosaicDefinitionTransactionBufferBuilder::new(&mut builder);
        txn_builder.add_size_(self.size() as u32);
        txn_builder.add_signature(abs_vector.signature_vec);
        txn_builder.add_signer(abs_vector.signer_vec);
        txn_builder.add_version(abs_vector.version_vec);
        txn_builder.add_type_(abs_vector.type_vec);
        txn_builder.add_max_fee(abs_vector.max_fee_vec);
        txn_builder.add_deadline(abs_vector.deadline_vec);

        txn_builder.add_mosaic_nonce(self.mosaic_nonce.to_dto());
        txn_builder.add_mosaic_id(mosaic_vec);
        txn_builder.add_flags(flags);
        txn_builder.add_divisibility(self.properties.divisibility);
        txn_builder.add_num_optional_properties(self.properties.optional_properties.len() as u8);
        txn_builder.add_optional_properties(fb::WIPOffset::new(property_vec));
        let t = txn_builder.finish();

        builder.finish(t, None);

        let buf = builder.finished_data();
        mosaic_definition_transaction_schema().serialize(&mut buf.to_vec())
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

impl fmt::Display for MosaicDefinitionTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}
