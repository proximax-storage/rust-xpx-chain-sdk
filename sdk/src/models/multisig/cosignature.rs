use crypto::Keypair;
use failure;

use crate::models::{
    account::Account,
    errors,
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
            .downcast::<AggregateTransaction>()
            .map_err(|_| failure::err_msg(errors::ERR_INVALID_AGGREGATE_TRANSACTION))?;

        Ok(Self(*aggregate))
    }

    pub(crate) fn sign_cosignature_transaction(
        &self,
        account: Account,
    ) -> crate::Result<CosignatureSignedTransaction> {
        ensure!(
            !self.0.transaction_hash().is_empty(),
            errors::ERR_EMPTY_COSIGNATURE_HASH
        );

        let key_pair: Keypair = Keypair::from_private_key(account.key_pair.secret);

        let hash_bytes = hex::decode(&self.0.transaction_hash())?;

        let signature = key_pair.sign(&hash_bytes);

        Ok(CosignatureSignedTransaction::new(
            self.0.transaction_hash(),
            Signature::new(signature.to_bytes()),
            account.public_account.public_key,
        ))
    }
}
