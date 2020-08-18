/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {crypto::Keypair, failure};

use crate::models::{
    account::Account,
    errors_const,
    transaction::{
        AbsTransaction, AggregateTransaction, CosignatureSignedTransaction, Signature, Transaction,
    },
};

/// Cosign an aggregate bonded transaction.
/// [CosignatureTransaction] are used to sign announced aggregate bonded transactions with missing cosignatures.
///
#[derive(Debug, Serialize)]
pub struct CosignatureTransaction(AggregateTransaction);

impl CosignatureTransaction {
    pub fn new(tx: Box<dyn Transaction>) -> crate::Result<Self> {
        let aggregate = tx
            .try_downcast::<AggregateTransaction>()
            .map_err(|_| failure::err_msg(errors_const::ERR_INVALID_AGGREGATE_TRANSACTION))?;

        Ok(Self(*aggregate))
    }

    pub(crate) fn sign_cosignature_transaction(
        &self,
        account: Account,
    ) -> crate::Result<CosignatureSignedTransaction> {
        ensure!(
            !self.0.transaction_hash().is_empty(),
            errors_const::ERR_EMPTY_COSIGNATURE_HASH
        );

        let signer = account.to_signer();
        let key_pair: Keypair = Keypair::from_private_key(account.key_pair.secret);

        let hash_bytes = self.0.transaction_hash().to_vec();

        let signature = key_pair.sign(&hash_bytes);

        Ok(CosignatureSignedTransaction::new(
            self.0.transaction_hash(),
            Signature::new(signature.to_bytes()),
            signer,
        ))
    }
}
