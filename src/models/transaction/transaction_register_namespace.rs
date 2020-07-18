/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {::std::fmt, failure::_core::any::Any, serde_json::Value};

use crate::{
    models::{
        account::{Account, PublicAccount},
        asset_id_model::AssetId,
        consts::REGISTER_NAMESPACE_HEADER_SIZE,
        errors_const,
        namespace::{generate_namespace_id, NamespaceId, NamespaceType},
        network::NetworkType,
        uint_64::Uint64,
    },
    Result,
};

use super::{
    buffer::register_namespace::buffers, internal::sign_transaction,
    schema::register_namespace_transaction_schema, AbsTransaction, AbstractTransaction, Deadline,
    EntityTypeEnum, SignedTransaction, Transaction, REGISTER_NAMESPACE_VERSION,
};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterNamespaceTransaction {
    pub abs_transaction: AbstractTransaction,
    pub namespace_type: NamespaceType,
    pub namespace_id: NamespaceId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Uint64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<NamespaceId>,
}

impl RegisterNamespaceTransaction {
    pub fn create_root(
        deadline: Deadline,
        namespace_name: &str,
        duration: Uint64,
        network_type: NetworkType,
    ) -> Result<RegisterNamespaceTransaction> {
        ensure!(
            !namespace_name.is_empty() && namespace_name.len() <= 16,
            errors_const::ERR_INVALID_NAMESPACE_NAME
        );

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            REGISTER_NAMESPACE_VERSION,
            EntityTypeEnum::NamespaceRegistration,
            network_type,
        );

        let namespace_id = NamespaceId::from_name(namespace_name)?;

        Ok(RegisterNamespaceTransaction {
            abs_transaction: abs_tx,
            namespace_type: NamespaceType::Root,
            namespace_id,
            name: namespace_name.parse().unwrap(),
            duration: Some(duration),
            parent_id: None,
        })
    }

    pub fn create_sub(
        deadline: Deadline,
        namespace_name: &'static str,
        parent_id: NamespaceId,
        network_type: NetworkType,
    ) -> Result<Self> {
        ensure!(
            !namespace_name.is_empty() && namespace_name.len() <= 64,
            errors_const::ERR_INVALID_NAMESPACE_NAME
        );

        ensure!(
            parent_id.to_u64() != 0,
            errors_const::ERR_EMPTY_NAMESPACE_ID
        );

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            REGISTER_NAMESPACE_VERSION,
            EntityTypeEnum::NamespaceRegistration,
            network_type,
        );

        let namespace_id = generate_namespace_id(namespace_name, parent_id)?;

        Ok(Self {
            abs_transaction: abs_tx,
            namespace_type: NamespaceType::Sub,
            namespace_id,
            name: namespace_name.parse().unwrap(),
            duration: None,
            parent_id: Some(parent_id),
        })
    }
}

impl AbsTransaction for RegisterNamespaceTransaction {
    fn abs_transaction(&self) -> AbstractTransaction {
        self.abs_transaction.to_owned()
    }
}

impl Transaction for RegisterNamespaceTransaction {
    fn size(&self) -> usize {
        REGISTER_NAMESPACE_HEADER_SIZE + self.name.len()
    }

    fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    fn sign_transaction_with(
        self,
        account: Account,
        generation_hash: String,
    ) -> Result<SignedTransaction> {
        sign_transaction(self, account, generation_hash)
    }

    fn embedded_to_bytes<'a>(&self) -> Result<Vec<u8>> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut builder = fb::FlatBufferBuilder::new();

        let namespace_id_vec = builder.create_vector(&self.namespace_id.to_u32_array());

        let d_vec = match self.namespace_type {
            NamespaceType::Root => builder.create_vector(&self.duration.unwrap().to_i32_array()),
            _ => builder.create_vector(&self.parent_id.unwrap().to_u32_array()),
        };

        let name_vec = builder.create_string(self.name.as_ref());

        let abs_vector = self.abs_transaction.build_vector(&mut builder);

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
        Ok(register_namespace_transaction_schema().serialize(&mut buf.to_vec()))
    }

    fn set_aggregate(&mut self, signer: PublicAccount) {
        self.abs_transaction.set_aggregate(signer)
    }

    fn as_any(&self) -> &dyn Any {
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
