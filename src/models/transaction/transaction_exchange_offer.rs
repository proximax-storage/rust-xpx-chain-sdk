/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {::std::fmt, failure::_core::any::Any, fb::FlatBufferBuilder, serde_json::Value};

use crate::{
    models::{
        account::{Account, PublicAccount},
        consts::{EXCHANGE_OFFER_HEADER_SIZE, EXCHANGE_OFFER_SIZE},
        errors_const,
        exchange::ExchangeConfirmation,
        network::NetworkType,
    },
    Result,
};
use crate::transaction::schema::exchange_offer_transaction_schema;

use super::{
    AbstractTransaction, AbsTransaction, buffer::exchange as buffer, deadline::Deadline,
    EXCHANGE_OFFER_VERSION, HashValue, internal::sign_transaction, SignedTransaction, Transaction,
    TransactionType,
};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeOfferTransaction {
    pub abs_transaction: AbstractTransaction,
    pub confirmations: Vec<ExchangeConfirmation>,
}

impl ExchangeOfferTransaction {
    pub fn new(
        deadline: Deadline,
        confirmations: Vec<ExchangeConfirmation>,
        network_type: NetworkType,
    ) -> Result<Self> {
        ensure!(!confirmations.is_empty(), errors_const::ERR_EMPTY_ADDRESSES);

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            EXCHANGE_OFFER_VERSION,
            TransactionType::ExchangeOffer,
            network_type,
        );

        Ok(Self {
            abs_transaction: abs_tx,
            confirmations,
        })
    }
}

impl AbsTransaction for ExchangeOfferTransaction {
    fn abs_transaction(&self) -> AbstractTransaction {
        self.abs_transaction.to_owned()
    }
}

impl Transaction for ExchangeOfferTransaction {
    fn size(&self) -> usize {
        EXCHANGE_OFFER_HEADER_SIZE + self.confirmations.len() * EXCHANGE_OFFER_SIZE
    }

    fn as_value(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    fn sign_transaction_with(
        self,
        account: Account,
        generation_hash: HashValue,
    ) -> Result<SignedTransaction> {
        sign_transaction(self, account, generation_hash)
    }

    fn embedded_to_bytes<'a>(&self) -> Result<Vec<u8>> {
        // Build up a serialized buffer algorithmically.
        // Initialize it with a capacity of 0 bytes.
        let mut builder = fb::FlatBufferBuilder::new();

        let abs_vector = self.abs_transaction.build_vector(&mut builder);

        let offers_vector =
            exchange_offer_to_array_to_buffer(&mut builder, self.confirmations.clone());

        let mut txn_builder = buffer::AddExchangeOfferTransactionBufferBuilder::new(&mut builder);

        txn_builder.add_size_(self.size() as u32);
        txn_builder.add_signature(abs_vector.signature_vec);
        txn_builder.add_signer(abs_vector.signer_vec);
        txn_builder.add_version(abs_vector.version_vec);
        txn_builder.add_type_(abs_vector.type_vec);
        txn_builder.add_max_fee(abs_vector.max_fee_vec);
        txn_builder.add_deadline(abs_vector.deadline_vec);
        txn_builder.add_offers_count(self.confirmations.len() as u8);
        txn_builder.add_offers(fb::WIPOffset::new(offers_vector));

        let t = txn_builder.finish();

        builder.finish(t, None);

        let buf = builder.finished_data();

        Ok(exchange_offer_transaction_schema().serialize(&mut buf.to_vec()))
    }

    fn set_aggregate(&mut self, signer: PublicAccount) {
        self.abs_transaction.set_aggregate(signer)
    }

    fn box_clone(&self) -> Box<dyn Transaction + 'static> {
        Box::new((*self).clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl fmt::Display for ExchangeOfferTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

pub(crate) fn exchange_offer_to_array_to_buffer<'a>(
    builder: &mut FlatBufferBuilder<'a>,
    offers: Vec<ExchangeConfirmation>,
) -> fb::UOffsetT {
    let mut offers_buffer: Vec<fb::WIPOffset<buffer::ExchangeOfferBuffer<'a>>> =
        Vec::with_capacity(offers.len());

    for item in offers {
        let mosaic_vector =
            builder.create_vector_direct(&item.offer.mosaic.asset_id.to_u32_array());

        let mosaic_amount_vector =
            builder.create_vector_direct(&item.offer.mosaic.amount.to_u32_array());

        let cost_vector = builder.create_vector_direct(&item.offer.cost.to_u32_array());

        let account_vector = builder.create_vector_direct(&item.owner.to_bytes());

        let mut add_exchange_offer = buffer::ExchangeOfferBufferBuilder::new(builder);
        add_exchange_offer.add_mosaic_id(mosaic_vector);
        add_exchange_offer.add_mosaic_amount(mosaic_amount_vector);
        add_exchange_offer.add_cost(cost_vector);
        add_exchange_offer.add_owner(account_vector);
        add_exchange_offer.add_type_(item.offer.r#type.value());

        offers_buffer.push(add_exchange_offer.finish());
    }

    builder.create_vector(&offers_buffer).value()
}
