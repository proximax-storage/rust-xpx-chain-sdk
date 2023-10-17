/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use ::std::fmt;
use std::any::Any;
use serde_json::Value;

use crate::{AsUint64, mosaic::{MosaicSupplyType, UnresolvedMosaicId}, network::NetworkType};
use crate::account::PublicAccount;
use crate::models::consts::MOSAIC_SUPPLY_CHANGE_TRANSACTION_SIZE;
use crate::transaction::buffers;
use crate::transaction::schema::mosaic_supply_change_transaction_schema;

use super::{
    CommonTransaction,
    deadline::Deadline, Transaction, TransactionType, TransactionVersion,
};

#[derive(Clone, Debug, Serialize, Builder, Deserialize)]
#[builder(create_empty = "empty", build_fn(error = "crate::api::error::Error"))]
#[serde(rename_all = "camelCase")]
pub struct MosaicSupplyChangeTransaction {
    /// Represents common transaction information..
    #[builder(private, pattern = "mutable")]
    pub common: CommonTransaction,
    pub supply_type: MosaicSupplyType,
    #[builder(setter(custom))]
    pub asset_id: Box<dyn UnresolvedMosaicId>,
    pub delta: u64,
}

impl MosaicSupplyChangeTransactionBuilder {
    /// The deadline method sets the deadline field.
    pub fn deadline(&mut self, value: Deadline) -> &mut MosaicSupplyChangeTransactionBuilder {
        self.common.as_mut().map(|item| item.deadline = Some(value));
        self
    }

    /// The max_fee method sets the max_fee field.
    pub fn max_fee(&mut self, value: u64) -> &mut MosaicSupplyChangeTransactionBuilder {
        self.common.as_mut().map(|item| item.max_fee = Some(value));
        self
    }

    /// The asset_id method sets the asset_id field.
    pub fn asset_id<M: 'static + UnresolvedMosaicId>(
        &mut self,
        value: M,
    ) -> &mut MosaicSupplyChangeTransactionBuilder {
        self.asset_id = Some(Box::new(value));
        self
    }
}

impl MosaicSupplyChangeTransaction {
    /// Build a MosaicSupplyChangeTransaction transaction object.
    pub fn builder(network_type: NetworkType) -> MosaicSupplyChangeTransactionBuilder {
        let common = CommonTransaction::create_from_type(
            TransactionType::MosaicSupplyChange,
            network_type,
            TransactionVersion::MOSAIC_SUPPLY_CHANGE,
            Some(Default::default()),
            None,
        );
        MosaicSupplyChangeTransactionBuilder { common: Some(common), ..Default::default() }
    }
}

#[typetag::serde]
impl Transaction for MosaicSupplyChangeTransaction {
    fn size(&self) -> usize {
        MOSAIC_SUPPLY_CHANGE_TRANSACTION_SIZE
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
        let mosaic_vec = builder.create_vector(&self.asset_id.to_dto());
        let delta_vec = builder.create_vector(&self.delta.to_dto());

        let abs_vector = self.common.build_vector(&mut builder);

        let mut txn_builder =
            buffers::MosaicSupplyChangeTransactionBufferBuilder::new(&mut builder);
        txn_builder.add_size_(self.size() as u32);
        txn_builder.add_signature(abs_vector.signature_vec);
        txn_builder.add_signer(abs_vector.signer_vec);
        txn_builder.add_version(abs_vector.version_vec);
        txn_builder.add_type_(abs_vector.type_vec);
        txn_builder.add_max_fee(abs_vector.max_fee_vec);
        txn_builder.add_deadline(abs_vector.deadline_vec);
        txn_builder.add_mosaic_id(mosaic_vec);
        txn_builder.add_direction(self.supply_type.clone() as u8);
        txn_builder.add_delta(delta_vec);
        let t = txn_builder.finish();

        builder.finish(t, None);

        let buf = builder.finished_data();
        mosaic_supply_change_transaction_schema().serialize(&mut buf.to_vec())
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

impl fmt::Display for MosaicSupplyChangeTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}
