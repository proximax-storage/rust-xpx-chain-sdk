use ::std::fmt;

use failure::_core::any::Any;
use serde_json::Value;

use crate::{fb, models::{
    {Id, Uint64},
    account::{Account, PublicAccount},
    consts::REGISTER_NAMESPACE_HEADER_SIZE,
    errors,
    namespace::{NamespaceId, NamespaceType},
    network::NetworkType
}
};
use crate::models::namespace::generate_namespace_id;

use super::{
    AbstractTransaction,
    buffer::register_namespace::buffers,
    Deadline,
    EntityTypeEnum,
    internal::sign_transaction,
    REGISTER_NAMESPACE_VERSION,
    schema::register_namespace_transaction_schema,
    SignedTransaction,
    Transaction
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterNamespaceTransaction {
    pub abs_transaction: AbstractTransaction,
    pub namespace_type: NamespaceType,
    pub namespace_id: NamespaceId,
    pub name: &'static str,
    pub duration: Uint64,
    pub parent_id: NamespaceId
}

impl RegisterNamespaceTransaction {
    pub fn create_root(
        deadline: Deadline,
        namespace_name: &'static str,
        duration: Uint64,
        network_type: NetworkType,
    ) -> crate::Result<RegisterNamespaceTransaction> {
        ensure!(
            namespace_name.len() != 0 && namespace_name.len() <= 16 ,
            errors::ERR_INVALID_NAMESPACE_NAME
        );

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            REGISTER_NAMESPACE_VERSION,
            EntityTypeEnum::NamespaceRegistration,
            network_type);

        let namespace_id = NamespaceId::from_name(namespace_name)?;

        Ok(RegisterNamespaceTransaction {
            abs_transaction: abs_tx,
            namespace_type: NamespaceType::Root,
            namespace_id,
            name: namespace_name,
            duration,
            parent_id: Default::default()
        })
    }

    pub fn create_sub(
        deadline: Deadline,
        namespace_name: &'static str,
        parent_id: NamespaceId,
        network_type: NetworkType,
    ) -> crate::Result<RegisterNamespaceTransaction> {
        ensure!(
            namespace_name.len() != 0 && namespace_name.len() <= 64 ,
            errors::ERR_INVALID_NAMESPACE_NAME
        );

        ensure!(
            parent_id.to_id().0 != 0,
            errors::ERR_NULL_NAMESPACE_ID
        );

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            REGISTER_NAMESPACE_VERSION,
            EntityTypeEnum::NamespaceRegistration,
            network_type);

        let namespace_id = generate_namespace_id(namespace_name, parent_id)?;

        Ok(RegisterNamespaceTransaction {
            abs_transaction: abs_tx,
            namespace_type: NamespaceType::Sub,
            namespace_id,
            name: namespace_name,
            duration: Default::default(),
            parent_id
        })
    }

    pub fn to_aggregate(&mut self, signer: PublicAccount) {
        self.abs_transaction.to_aggregate(signer)
    }

    pub fn is_unconfirmed(&self) -> bool {
        self.abs_transaction.is_unconfirmed()
    }

    pub fn is_confirmed(&self) -> bool {
        self.abs_transaction.is_confirmed()
    }

    pub fn has_missing_signatures(&self) -> bool {
        self.abs_transaction.has_missing_signatures()
    }
}

impl Transaction for RegisterNamespaceTransaction {
    fn get_abs_transaction(self) -> AbstractTransaction {
        self.abs_transaction
    }

    fn size(&self) -> usize {
        REGISTER_NAMESPACE_HEADER_SIZE + self.name.len()
    }

    fn generate_bytes<'a>(&self) -> Vec<u8> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut builder = fb::FlatBufferBuilder::new();

        let namespace_id_vec = builder.create_vector(&self.namespace_id.to_int_array());

        let mut d_vec = fb::WIPOffset::new(0);
        if self.namespace_type == NamespaceType::Root {
            d_vec = builder.create_vector(&self.duration.to_int_array());
        } else {
            d_vec = builder.create_vector(&self.parent_id.to_int_array());
        }

        let name_vec = builder.create_string(self.name);

        let abs_vector = &self.abs_transaction.generate_vector(&mut builder);

        let mut txn_builder =
            buffers::RegisterNamespaceTransactionBufferBuilder::new(&mut builder);
        txn_builder.add_size_(self.size() as u32);
        txn_builder.add_signature(fb::WIPOffset::new(*abs_vector.get("signatureV").unwrap()));
        txn_builder.add_signer(fb::WIPOffset::new(*abs_vector.get("signerV").unwrap()));
        txn_builder.add_version(*abs_vector.get("versionV").unwrap());
        txn_builder.add_type_(self.abs_transaction.transaction_type.get_value());
        txn_builder.add_max_fee(fb::WIPOffset::new(*abs_vector.get("feeV").unwrap()));
        txn_builder.add_deadline(fb::WIPOffset::new(*abs_vector.get("deadlineV").unwrap()));

        txn_builder.add_namespace_type(self.namespace_type.clone() as u8);
        txn_builder.add_duration_parent_id(d_vec);
        txn_builder.add_namespace_id(namespace_id_vec);
        txn_builder.add_namespace_name_size(self.name.len() as u8);
        txn_builder.add_namespace_name(name_vec);

        let t = txn_builder.finish();

        builder.finish(t, None);

        let buf = builder.finished_data();
        register_namespace_transaction_schema().serialize(&mut Vec::from(buf))
    }

    fn generate_embedded_bytes(&self) -> Vec<u8> {
        unimplemented!()
    }

    fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    fn has_missing_signatures(&self) -> bool {
        unimplemented!()
    }

    fn sign_transaction_with(&self, account: Account, generation_hash: String)
                             -> crate::Result<SignedTransaction> {
        sign_transaction(self as &dyn Transaction, account, generation_hash)
    }

    fn entity_type(&self) -> EntityTypeEnum {
        self.abs_transaction.transaction_type.to_owned()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl fmt::Display for RegisterNamespaceTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
