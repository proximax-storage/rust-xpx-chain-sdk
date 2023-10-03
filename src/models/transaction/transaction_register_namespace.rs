/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use anyhow::Result;

use ::std::fmt;
use std::any::Any;
use serde_json::Value;

use crate::{AsUint64, errors_const, mosaic::UnresolvedMosaicId, namespace::{generate_namespace_id, NamespaceId, NamespaceType}, network::NetworkType};
use crate::models::consts::REGISTER_NAMESPACE_HEADER_SIZE;
use crate::transaction::buffers;
use crate::transaction::schema::register_namespace_transaction_schema;

use super::{
    CommonTransaction, Deadline,
    Transaction, TransactionType, TransactionVersion,
};

#[derive(Clone, Debug, Serialize, Builder, Deserialize)]
#[builder(
create_empty = "empty",
build_fn(validate = "Self::validate", error = "crate::api::error::Error")
)]
#[serde(rename_all = "camelCase")]
pub struct RegisterNamespaceTransaction {
    /// Represents common transaction information..
    #[builder(private, pattern = "mutable")]
    pub common: CommonTransaction,
    pub namespace_type: NamespaceType,
    pub namespace_id: NamespaceId,
    #[builder(private)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    pub duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    pub parent_id: Option<NamespaceId>,
}

impl RegisterNamespaceTransactionBuilder {
    fn validate(&self) -> Result<()> {
        let namespace_name =
            self.name.clone().ok_or(anyhow!(errors_const::ERR_EMPTY_NAMESPACE_NAME))?;

        ensure!(!namespace_name.is_empty(), errors_const::ERR_EMPTY_NAMESPACE_NAME);

        if self.namespace_type == Some(NamespaceType::Root) {
            ensure!(namespace_name.len() <= 16, errors_const::ERR_INVALID_LEN_NAMESPACE_NAME);
        } else if self.namespace_type == Some(NamespaceType::Sub) {
            ensure!(namespace_name.len() <= 64, errors_const::ERR_INVALID_LEN_NAMESPACE_NAME);
            let parent_id = self
                .parent_id
                .unwrap_or_default()
                .ok_or(anyhow!(errors_const::ERR_EMPTY_NAMESPACE_ID))?;

            ensure!(parent_id.to_u64() != 0, errors_const::ERR_EMPTY_NAMESPACE_ID);
        }

        Ok(())
    }

    /// The deadline method sets the deadline field.
    pub fn deadline(&mut self, value: Deadline) -> &mut RegisterNamespaceTransactionBuilder {
        self.common.as_mut().map(|item| item.deadline = Some(value));
        self
    }

    /// The max_fee method sets the max_fee field.
    pub fn max_fee(&mut self, value: u64) -> &mut RegisterNamespaceTransactionBuilder {
        self.common.as_mut().map(|item| item.max_fee = Some(value));
        self
    }
}

impl RegisterNamespaceTransaction {
    pub fn builder_root(
        namespace_name: impl Into<String>,
        network_type: NetworkType,
    ) -> RegisterNamespaceTransactionBuilder {
        let common = CommonTransaction::create_from_type(
            TransactionType::RegisterNamespace,
            network_type,
            TransactionVersion::NAMESPACE_REGISTRATION,
            Some(Default::default()),
            None,
        );

        let namespace_name = namespace_name.into();

        let namespace_id = NamespaceId::from_name(namespace_name.as_str()).unwrap_or_default();

        RegisterNamespaceTransactionBuilder {
            common: Some(common),
            namespace_id: Some(namespace_id),
            name: Some(namespace_name),
            namespace_type: Some(NamespaceType::Root),
            ..Default::default()
        }
    }

    pub fn builder_sub(
        namespace_name: impl Into<String>,
        parent_id: NamespaceId,
        network_type: NetworkType,
    ) -> RegisterNamespaceTransactionBuilder {
        let common = CommonTransaction::create_from_type(
            TransactionType::RegisterNamespace,
            network_type,
            TransactionVersion::NAMESPACE_REGISTRATION,
            Some(Default::default()),
            None,
        );

        let namespace_name = namespace_name.into();

        let namespace_id =
            generate_namespace_id(namespace_name.as_str(), parent_id).unwrap_or_default();

        RegisterNamespaceTransactionBuilder {
            common: Some(common),
            namespace_id: Some(namespace_id),
            name: Some(namespace_name),
            parent_id: Some(Some(parent_id)),
            duration: Some(Default::default()),
            namespace_type: Some(NamespaceType::Sub),
            ..Default::default()
        }
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
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}
