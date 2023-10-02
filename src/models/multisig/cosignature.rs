/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use anyhow::Result;
use crypto::{Keypair, Signature};

use crate::{
    account::Account,
    errors_const,
    transaction::{AggregateTransaction, CosignatureSignedTransaction, Transaction},
};

/// Cosign an aggregate bonded transaction.
/// [CosignatureTransaction] are used to sign announced aggregate bonded transactions with missing cosignatures.
///
#[derive(Debug, Serialize)]
pub struct CosignatureTransaction(AggregateTransaction);

impl CosignatureTransaction {
    pub fn create(tx: Box<dyn Transaction>) -> Result<Self> {
        let aggregate = tx
            .try_downcast::<AggregateTransaction>()
            .map_err(|_| anyhow!(errors_const::ERR_INVALID_AGGREGATE_TRANSACTION))?;

        Ok(Self(*aggregate))
    }

    pub fn sign_cosignature_transaction(
        &self,
        account: Account,
    ) -> Result<CosignatureSignedTransaction> {
        ensure!(!self.0.get_transaction_hash().is_zero(), errors_const::ERR_EMPTY_COSIGNATURE_HASH);

        let signer = account.to_signer();
        let key_pair: Keypair = Keypair::from_private_key(account.key_pair.secret);

        let hash_bytes = self.0.get_transaction_hash();

        let signature = key_pair.sign(hash_bytes.as_bytes());

        Ok(CosignatureSignedTransaction::create(
            self.0.get_transaction_hash(),
            Signature::from_bytes(&signature.to_bytes()).map_err(|err| anyhow!(err))?,
            signer,
        ))
    }
}
