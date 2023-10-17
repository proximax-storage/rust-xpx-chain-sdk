/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::any::Any;
use anyhow::Result;
use serde_json::Value;

use {::std::fmt, fb::FlatBufferBuilder};
use crate::account::PublicAccount;

use crate::models::{
    errors_const,
    exchange::RemoveOffer,
    network::NetworkType,
};
use crate::models::consts::{REMOVE_EXCHANGE_OFFER_HEADER_SIZE, REMOVE_EXCHANGE_OFFER_SIZE};
use crate::transaction::schema::remove_exchange_offer_transaction_schema;

use super::{
    buffers, CommonTransaction,
    deadline::Deadline, Transaction, TransactionType, TransactionVersion,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveExchangeOfferTransaction {
    pub common: CommonTransaction,
    pub offers: Vec<RemoveOffer>,
}

impl RemoveExchangeOfferTransaction {
    pub fn create(
        deadline: Deadline,
        offers: Vec<RemoveOffer>,
        network_type: NetworkType,
        max_fee: Option<u64>,
    ) -> Result<Self> {
        ensure!(!offers.is_empty(), errors_const::ERR_EMPTY_ADDRESSES);

        let common = CommonTransaction::create_from_type(
            TransactionType::RemoveExchangeOffer,
            network_type,
            TransactionVersion::REMOVE_EXCHANGE_OFFER,
            Some(deadline),
            max_fee,
        );

        Ok(Self { common, offers })
    }
}

#[typetag::serde]
impl Transaction for RemoveExchangeOfferTransaction {
    fn size(&self) -> usize {
        REMOVE_EXCHANGE_OFFER_HEADER_SIZE + self.offers.len() * REMOVE_EXCHANGE_OFFER_SIZE
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

        let abs_vector = self.common.build_vector(&mut builder);

        let offers_vector =
            remove_exchange_offer_to_array_to_buffer(&mut builder, self.offers.clone());

        let mut txn_builder =
            buffers::RemoveExchangeOfferTransactionBufferBuilder::new(&mut builder);

        txn_builder.add_size_(self.size() as u32);
        txn_builder.add_signature(abs_vector.signature_vec);
        txn_builder.add_signer(abs_vector.signer_vec);
        txn_builder.add_version(abs_vector.version_vec);
        txn_builder.add_type_(abs_vector.type_vec);
        txn_builder.add_max_fee(abs_vector.max_fee_vec);
        txn_builder.add_deadline(abs_vector.deadline_vec);
        txn_builder.add_offers_count(self.offers.len() as u8);
        txn_builder.add_offers(fb::WIPOffset::new(offers_vector));

        let t = txn_builder.finish();

        builder.finish(t, None);

        let buf = builder.finished_data();

        remove_exchange_offer_transaction_schema().serialize(&mut buf.to_vec())
    }

    fn set_aggregate(&mut self, signer: PublicAccount) {
        self.common.set_aggregate(signer)
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

impl fmt::Display for RemoveExchangeOfferTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}

pub(crate) fn remove_exchange_offer_to_array_to_buffer<'a>(
    builder: &mut FlatBufferBuilder<'a>,
    offers: Vec<RemoveOffer>,
) -> fb::UOffsetT {
    let mut offers_buffer: Vec<fb::WIPOffset<buffers::RemoveExchangeOfferBuffer<'a>>> =
        Vec::with_capacity(offers.len());

    for item in offers {
        let mosaic_vector = builder.create_vector(&item.asset_id.to_dto());

        let mut add_exchange_offer = buffers::RemoveExchangeOfferBufferBuilder::new(builder);
        add_exchange_offer.add_mosaic_id(mosaic_vector);
        add_exchange_offer.add_type_(item.r#type.value());

        offers_buffer.push(add_exchange_offer.finish());
    }

    builder.create_vector(&offers_buffer).value()
}
