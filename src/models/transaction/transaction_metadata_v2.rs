/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::any::{Any, TypeId};
use std::fmt;
use std::str::FromStr;

use crate::{AsUint64, errors_const};
use crate::models::{account::PublicAccount, consts::METADATA_V2_HEADER_SIZE};

use super::{buffers, CommonTransaction, schema::modify_metadata_v2_transaction_schema};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataTransaction {
    pub common: CommonTransaction,
    pub target_public_account: PublicAccount,
    pub scoped_metadata_key: u64,
    pub value_size_delta: u16,
    pub value: String,
}

impl MetadataTransaction {
    pub(crate) fn builder(
        common: CommonTransaction,
        account: PublicAccount,
        new_value: &str,
        old_value: &str,
        scoped_metadata_key: u64,
    ) -> anyhow::Result<MetadataTransaction> {
        let mut metadata = MetadataTransaction {
            common,
            target_public_account: account,
            scoped_metadata_key,
            value_size_delta: (new_value.len() - old_value.len()) as u16,
            value: new_value.to_owned(),
        };

        let value = factory_value(new_value, old_value);

        metadata.value = String::from_utf8(value)?;
        Ok(metadata)
    }

    pub(crate) fn size(&self) -> usize {
        METADATA_V2_HEADER_SIZE + self.value.len()
    }

    pub(crate) fn set_aggregate(&mut self, signer: PublicAccount) {
        self.common.set_aggregate(signer)
    }

    pub(crate) fn embedded_to_bytes(
        &self,
        builder: &mut fb::FlatBufferBuilder,
        target_id: fb::WIPOffset<fb::Vector<u8>>,
        size: usize,
    ) -> Vec<u8> {
        let target_key_offset = builder.create_vector(self.target_public_account.to_builder());

        let scoped_metadata_key_offset = builder.create_vector(&self.scoped_metadata_key.to_dto());

        let value_offset = builder.create_vector(self.value.as_bytes());

        let value_size_offset = builder.create_vector(&(self.value.len() as u16).to_le_bytes());

        let value_size_delta_offset = builder.create_vector(&self.value_size_delta.to_le_bytes());

        let abs_vector = self.common.build_vector(builder);

        let mut txn_builder = buffers::MetadataV2TransactionBufferBuilder::new(builder);
        txn_builder.add_size_(size as u32);
        txn_builder.add_signature(abs_vector.signature_vec);
        txn_builder.add_signer(abs_vector.signer_vec);
        txn_builder.add_version(abs_vector.version_vec);
        txn_builder.add_type_(abs_vector.type_vec);
        txn_builder.add_max_fee(abs_vector.max_fee_vec);
        txn_builder.add_deadline(abs_vector.deadline_vec);
        txn_builder.add_target_key(target_key_offset);
        txn_builder.add_scoped_metadata_key(scoped_metadata_key_offset);
        txn_builder.add_value(value_offset);
        txn_builder.add_value_size(value_size_offset);
        txn_builder.add_value_size_delta(value_size_delta_offset);
        txn_builder.add_target_id(target_id);

        let t = txn_builder.finish();

        builder.finish(t, None);

        let buf = builder.finished_data();

        modify_metadata_v2_transaction_schema().serialize(&mut buf.to_vec())
    }
}

pub(crate) fn build_scoped_metadata_key<T: fmt::Display + 'static>(
    scoped_key: T,
) -> anyhow::Result<u64> {
    Ok(if TypeId::of::<&str>() == scoped_key.type_id() {
        u64::from_utf8_str(&scoped_key.to_string())
    } else if TypeId::of::<u64>() == scoped_key.type_id() {
        u64::from_str(&scoped_key.to_string())?
    } else {
        return Err(anyhow!(errors_const::ERR_METADATA_INVALID_SCOPED_METADATA_KEY_TYPE));
    })
}

pub(crate) fn factory_value(new_value: &str, old_value: &str) -> Vec<u8> {
    let mut new_value = new_value.to_owned();
    let mut old_value = old_value.to_owned();

    if new_value.len() < old_value.len() {
        let value = new_value.clone();
        let new_old_value = old_value.clone();
        old_value = value;
        new_value = new_old_value;
    }

    let mut value = new_value.as_bytes().to_vec();

    for (i, c) in old_value.to_owned().into_bytes().into_iter().enumerate() {
        value[i] ^= c
    }
    value
}
