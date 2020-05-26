// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use std::any::Any;
use std::fmt;

use serde_json::Value;

use crate::models::{
    account::{Account, PublicAccount},
    alias::AliasActionType,
    asset_id_model::AssetId,
    consts::MOSAIC_ID_SIZE,
    errors,
    mosaic::MosaicId,
    namespace::NamespaceId,
    network::NetworkType,
};
use crate::Result;

use super::{
    AbstractTransaction, AbsTransaction, AliasTransaction,
    Deadline, EntityTypeEnum, MOSAIC_ALIAS_VERSION, sign_transaction,
    SignedTransaction, Transaction,
};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MosaicAliasTransaction {
    pub alias_transaction: AliasTransaction,
    pub mosaic_id: MosaicId,
}

impl MosaicAliasTransaction {
    pub fn new(deadline: Deadline, mosaic_id: MosaicId, namespace_id: NamespaceId,
               action_type: AliasActionType, network_type: NetworkType) -> Result<Self>
    {
        ensure!(
            !mosaic_id.is_empty(),
            errors::ERR_EMPTY_MOSAIC_ID
         );

        ensure!(
            !namespace_id.is_empty(),
            errors::ERR_EMPTY_NAMESPACE_ID
         );

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            MOSAIC_ALIAS_VERSION,
            EntityTypeEnum::MosaicAlias,
            network_type,
        );

        Ok(Self {
            alias_transaction: AliasTransaction {
                abs_transaction: abs_tx,
                action_type,
                namespace_id,
            },
            mosaic_id,
        })
    }
}

impl AbsTransaction for MosaicAliasTransaction {
    fn abs_transaction(&self) -> AbstractTransaction {
        self.alias_transaction.abs_transaction()
    }
}

impl Transaction for MosaicAliasTransaction {
    fn size(&self) -> usize {
        self.alias_transaction.size() + MOSAIC_ID_SIZE
    }

    fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    fn sign_transaction_with(self, account: Account, generation_hash: String)
                             -> Result<SignedTransaction> {
        sign_transaction(self, account, generation_hash)
    }

    fn embedded_to_bytes(&self) -> Result<Vec<u8>> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut builder = fb::FlatBufferBuilder::new();

        let mosaic_bytes = self.mosaic_id.to_bytes();

        let mosaic_vector = builder.create_vector_direct(&mosaic_bytes);

        self.alias_transaction.embedded_to_bytes(&mut builder, mosaic_vector, MOSAIC_ID_SIZE)
    }

    fn to_aggregate(&mut self, signer: PublicAccount) {
        self.alias_transaction.to_aggregate(signer)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn box_clone(&self) -> Box<dyn Transaction + 'static> {
        Box::new((*self).clone())
    }
}

impl fmt::Display for MosaicAliasTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}