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
    errors_const,
    metadata::{MetadataModification, MetadataType},
    mosaic::MosaicId,
    network::NetworkType,
};
use crate::account::PublicAccount;
use crate::models::consts::MOSAIC_ID_SIZE;
use crate::mosaic::UnresolvedMosaicId;

use super::{
    CommonTransaction, Deadline, ModifyMetadataTransaction, Transaction, TransactionType,
    TransactionVersion,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataMosaicTransaction {
    pub metadata_transaction: ModifyMetadataTransaction,
    pub mosaic_id: MosaicId,
}

impl MetadataMosaicTransaction {
    pub fn new(
        deadline: Deadline,
        mosaic_id: MosaicId,
        modifications: Vec<MetadataModification>,
        network_type: NetworkType,
        max_fee: Option<u64>,
    ) -> Result<Self> {
        ensure!(!modifications.is_empty(), errors_const::ERR_METADATA_EMPTY_MODIFICATIONS);

        let common = CommonTransaction::create_from_type(
            TransactionType::ModifyMetadataMosaic,
            network_type,
            TransactionVersion::MOSAIC_METADATA,
            Some(deadline),
            max_fee,
        );

        Ok(Self {
            metadata_transaction: ModifyMetadataTransaction {
                common,
                metadata_type: MetadataType::MetadataMosaicType,
                modifications,
            },
            mosaic_id,
        })
    }
}

#[typetag::serde]
impl Transaction for MetadataMosaicTransaction {
    fn size(&self) -> usize {
        self.metadata_transaction.size() + MOSAIC_ID_SIZE
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

        let mosaic_id_bytes = self.mosaic_id.to_builder();

        let mosaic_id_vector = builder.create_vector(&mosaic_id_bytes);

        self.metadata_transaction
            .embedded_to_bytes(&mut builder, mosaic_id_vector, MOSAIC_ID_SIZE)
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

impl fmt::Display for MetadataMosaicTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}
