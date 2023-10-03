/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::any::Any;
use std::fmt;

use anyhow::Result;
use serde_json::Value;

use crate::account::PublicAccount;
use crate::errors_const;
use crate::models::consts::MOSAICS_SIZE;
use crate::mosaic::{MosaicId, UnresolvedMosaicId};
use crate::network::NetworkType;
use crate::transaction::{
    build_scoped_metadata_key, CommonTransaction, Deadline, MetadataTransaction, Transaction,
    TransactionType, TransactionVersion,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataV2MosaicTransaction {
    pub(crate) metadata: MetadataTransaction,
    pub(crate) mosaic_id: MosaicId,
}

impl MetadataV2MosaicTransaction {
    pub fn builder<T: fmt::Display + 'static>(
        deadline: Deadline,
        account: PublicAccount,
        mosaic_id: MosaicId,
        scoped_key: T,
        new_value: &str,
        old_value: Option<&str>,
        network_type: NetworkType,
        max_fee: Option<u64>,
    ) -> Result<Self> {
        if let Some(old) = old_value {
            ensure!(new_value != old, errors_const::ERR_METADATA_VALUE_IS_THE_SAME);
        }

        let scoped_metadata_key = build_scoped_metadata_key(scoped_key)?;

        let common = CommonTransaction::create_from_type(
            TransactionType::MosaicMetadataV2,
            network_type,
            TransactionVersion::MOSAIC_METADATA_V2,
            Some(deadline),
            max_fee,
        );

        let old_value = old_value.unwrap_or_default();

        let metadata = MetadataTransaction::builder(
            common,
            account,
            new_value,
            old_value,
            scoped_metadata_key,
        )?;

        Ok(Self { metadata, mosaic_id })
    }
}

#[typetag::serde]
impl Transaction for MetadataV2MosaicTransaction {
    fn size(&self) -> usize {
        MOSAICS_SIZE + self.metadata.size()
    }

    fn as_value(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    fn get_common_transaction(&self) -> CommonTransaction {
        self.metadata.common.clone()
    }

    fn to_serializer<'a>(&self) -> Vec<u8> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut builder = fb::FlatBufferBuilder::new();

        let target_id = builder.create_vector(&self.mosaic_id.to_builder());
        self.metadata.embedded_to_bytes(&mut builder, target_id, self.size())
    }

    fn set_aggregate(&mut self, signer: PublicAccount) {
        self.metadata.set_aggregate(signer)
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

impl fmt::Display for MetadataV2MosaicTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}
