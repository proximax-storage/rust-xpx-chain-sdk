/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::any::Any;

use anyhow::Result;

use {::std::fmt, serde_json::Value};

use crate::{
    errors_const,
    models::consts::REGISTER_NAMESPACE_HEADER_SIZE,
    mosaic::UnresolvedMosaicId,
    namespace::{generate_namespace_id, NamespaceId, NamespaceType},
    network::NetworkType,
    AsUint64,
};

use super::{
    buffers, schema::register_namespace_transaction_schema, CommonTransaction, Deadline,
    Transaction, TransactionType, TransactionVersion,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterNamespaceTransaction {
    pub common: CommonTransaction,
    pub namespace_type: NamespaceType,
    pub namespace_id: NamespaceId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<NamespaceId>,
}

impl RegisterNamespaceTransaction {
    pub fn create_root(
        deadline: Deadline,
        namespace_name: &str,
        duration: u64,
        network_type: NetworkType,
        max_fee: Option<u64>,
    ) -> Result<RegisterNamespaceTransaction> {
        ensure!(
            !namespace_name.is_empty() && namespace_name.len() <= 16,
            errors_const::ERR_INVALID_NAMESPACE_NAME
        );

        let common = CommonTransaction::create_from_type(
            TransactionType::RegisterNamespace,
            network_type,
            TransactionVersion::NAMESPACE_REGISTRATION,
            Some(deadline),
            max_fee,
        );

        let namespace_id = NamespaceId::from_name(namespace_name)?;

        Ok(RegisterNamespaceTransaction {
            common,
            namespace_type: NamespaceType::Root,
            namespace_id,
            name: namespace_name.to_owned(),
            duration: Some(duration),
            parent_id: None,
        })
    }

    pub fn create_sub(
        deadline: Deadline,
        namespace_name: &str,
        parent_id: NamespaceId,
        network_type: NetworkType,
        max_fee: Option<u64>,
    ) -> Result<Self> {
        ensure!(
            !namespace_name.is_empty() && namespace_name.len() <= 64,
            errors_const::ERR_INVALID_NAMESPACE_NAME
        );

        ensure!(
            parent_id.to_u64() != 0,
            errors_const::ERR_EMPTY_NAMESPACE_ID
        );

        let common = CommonTransaction::create_from_type(
            TransactionType::RegisterNamespace,
            network_type,
            TransactionVersion::NAMESPACE_REGISTRATION,
            Some(deadline),
            max_fee,
        );

        let namespace_id = generate_namespace_id(namespace_name, parent_id)?;

        Ok(Self {
            common,
            namespace_type: NamespaceType::Sub,
            namespace_id,
            name: namespace_name.to_owned(),
            duration: None,
            parent_id: Some(parent_id),
        })
    }
}

#[typetag::serde]
impl Transaction for RegisterNamespaceTransaction {
    fn size(&self) -> usize {
        REGISTER_NAMESPACE_HEADER_SIZE + self.name.len()
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

        let namespace_id_vec = builder.create_vector(&self.namespace_id.to_dto());

        let d_vec = match self.namespace_type {
            NamespaceType::Root => builder.create_vector(&self.duration.unwrap().to_dto()),
            _ => builder.create_vector(&self.parent_id.unwrap().to_dto()),
        };

        let name_vec = builder.create_string(self.name.as_ref());

        let abs_vector = self.common.build_vector(&mut builder);

        let mut txn_builder = buffers::RegisterNamespaceTransactionBufferBuilder::new(&mut builder);
        txn_builder.add_size_(self.size() as u32);
        txn_builder.add_signature(abs_vector.signature_vec);
        txn_builder.add_signer(abs_vector.signer_vec);
        txn_builder.add_version(abs_vector.version_vec);
        txn_builder.add_type_(abs_vector.type_vec);
        txn_builder.add_max_fee(abs_vector.max_fee_vec);
        txn_builder.add_deadline(abs_vector.deadline_vec);

        txn_builder.add_namespace_type(self.namespace_type.clone() as u8);
        txn_builder.add_duration_parent_id(d_vec);
        txn_builder.add_namespace_id(namespace_id_vec);
        txn_builder.add_namespace_name_size(self.name.len() as u8);
        txn_builder.add_namespace_name(name_vec);

        let t = txn_builder.finish();

        builder.finish(t, None);

        let buf = builder.finished_data();
        register_namespace_transaction_schema().serialize(&mut buf.to_vec())
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

impl fmt::Display for RegisterNamespaceTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
