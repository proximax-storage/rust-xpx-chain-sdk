/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {::std::fmt, failure::_core::any::Any, serde_json::Value};

use crate::{
    models::{
        account::{Account, PublicAccount},
        consts::{REMOVE_EXCHANGE_OFFER_HEADER_SIZE, REMOVE_EXCHANGE_OFFER_SIZE},
        errors_const,
        exchange::RemoveOffer,
        network::NetworkType,
    },
    Result,
};

use super::{
    deadline::Deadline, internal::sign_transaction, AbsTransaction, AbstractTransaction,
    EntityTypeEnum, SignedTransaction, Transaction, REMOVE_EXCHANGE_OFFER_VERSION,
};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveExchangeOfferTransaction {
    pub abs_transaction: AbstractTransaction,
    pub offers: Vec<RemoveOffer>,
}

impl RemoveExchangeOfferTransaction {
    pub fn new(
        deadline: Deadline,
        offers: Vec<RemoveOffer>,
        network_type: NetworkType,
    ) -> Result<Self> {
        ensure!(!offers.is_empty(), errors_const::ERR_EMPTY_ADDRESSES);

        let abs_tx = AbstractTransaction::new_from_type(
            deadline,
            REMOVE_EXCHANGE_OFFER_VERSION,
            EntityTypeEnum::RemoveExchangeOffer,
            network_type,
        );

        Ok(Self {
            abs_transaction: abs_tx,
            offers,
        })
    }
}

impl AbsTransaction for RemoveExchangeOfferTransaction {
    fn abs_transaction(&self) -> AbstractTransaction {
        self.abs_transaction.to_owned()
    }
}

impl Transaction for RemoveExchangeOfferTransaction {
    fn size(&self) -> usize {
        REMOVE_EXCHANGE_OFFER_HEADER_SIZE + self.offers.len() * REMOVE_EXCHANGE_OFFER_SIZE
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
        unimplemented!()
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

impl fmt::Display for RemoveExchangeOfferTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
