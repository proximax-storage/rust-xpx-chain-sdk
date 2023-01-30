/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::models::{
    account::PublicAccount,
    consts::METADATA_HEADER_SIZE,
    metadata::{MetadataModification, MetadataType},
};

use super::{
    buffers, CommonTransaction,
    internal::metadata_modification_array_to_buffer, schema::modify_metadata_transaction_schema,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyMetadataTransaction {
    pub common: CommonTransaction,
    pub metadata_type: MetadataType,
    pub modifications: Vec<MetadataModification>,
}

impl ModifyMetadataTransaction {
    pub(crate) fn set_aggregate(&mut self, signer: PublicAccount) {
        self.common.set_aggregate(signer)
    }

    pub(crate) fn common(&self) -> CommonTransaction {
        self.common.to_owned()
    }

    pub(crate) fn size(&self) -> usize {
        let size_of_modifications: usize = self.modifications.iter().map(|m| m.size()).sum();

        METADATA_HEADER_SIZE + size_of_modifications
    }

    pub(crate) fn embedded_to_bytes(
        &self,
        builder: &mut fb::FlatBufferBuilder,
        metadata_vec: fb::WIPOffset<fb::Vector<u8>>,
        alias_size: usize,
    ) -> Vec<u8> {
        let modification_vec =
            metadata_modification_array_to_buffer(builder, self.modifications.to_owned());

        let abs_vector = self.common.build_vector(builder);

        let mut txn_builder = buffers::ModifyMetadataTransactionBufferBuilder::new(builder);
        txn_builder.add_size_((self.size() + alias_size) as u32);
        txn_builder.add_signature(abs_vector.signature_vec);
        txn_builder.add_signer(abs_vector.signer_vec);
        txn_builder.add_version(abs_vector.version_vec);
        txn_builder.add_type_(abs_vector.type_vec);
        txn_builder.add_max_fee(abs_vector.max_fee_vec);
        txn_builder.add_deadline(abs_vector.deadline_vec);
        txn_builder.add_metadata_type(self.metadata_type.value());
        txn_builder.add_metadata_id(metadata_vec);
        txn_builder.add_modifications(fb::WIPOffset::new(modification_vec));
        let t = txn_builder.finish();

        builder.finish(t, None);

        let buf = builder.finished_data();

        modify_metadata_transaction_schema().serialize(&mut buf.to_vec())
    }
}
