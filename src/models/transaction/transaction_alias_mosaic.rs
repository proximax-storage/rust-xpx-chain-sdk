/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::any::Any;
use std::fmt;
use serde_json::Value;

use crate::{
    alias::AliasActionType,
    mosaic::{MosaicId, UnresolvedMosaicId},
    namespace::NamespaceId,
    network::NetworkType,
};
use crate::account::PublicAccount;
use crate::models::consts::MOSAIC_ID_SIZE;

use super::{
    AliasTransaction, CommonTransaction, Deadline, Transaction, TransactionType, TransactionVersion,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MosaicAliasTransaction {
    pub alias_transaction: AliasTransaction,
    pub mosaic_id: MosaicId,
}

impl MosaicAliasTransaction {
    pub fn create(
        deadline: Deadline,
        mosaic_id: MosaicId,
        namespace_id: NamespaceId,
        action_type: AliasActionType,
        network_type: NetworkType,
        max_fee: Option<u64>,
    ) -> Self {
        let abs_tx = CommonTransaction::create_from_type(
            TransactionType::MosaicAlias,
            network_type,
            TransactionVersion::MOSAIC_ALIAS,
            Some(deadline),
            max_fee,
        );

        Self {
            alias_transaction: AliasTransaction { common: abs_tx, action_type, namespace_id },
            mosaic_id,
        }
    }
}

#[typetag::serde]
impl Transaction for MosaicAliasTransaction {
    fn size(&self) -> usize {
        self.alias_transaction.size() + MOSAIC_ID_SIZE
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

        let mosaic_bytes = self.mosaic_id.to_builder();

        let mosaic_vector = builder.create_vector(&mosaic_bytes);

        self.alias_transaction
            .to_serializer(&mut builder, mosaic_vector, MOSAIC_ID_SIZE)
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

impl fmt::Display for MosaicAliasTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}
