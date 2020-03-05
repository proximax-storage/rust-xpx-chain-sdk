use crate::models::{
    errors,
    network::NetworkType,
    transaction::{SignedTransaction, Transaction},
    utils::{is_hex, vec_u8_to_hex}
};
use crate::Result;

use super::PublicAccount;

/// The `Account` account structure contains account's `PublicAccount` and private key.
#[derive(Debug, Clone)]
pub struct Account {
    /// The keyPair containing the public and private key of this account.
    pub key_pair: xpx_crypto::Keypair,
    /// The public account of this account.
    pub public_account: PublicAccount,
}

impl Account {
    /// Create a `Account` from a private key for the given `NetworkType`.
    pub fn from_private_key(private_key: &str, network_type: NetworkType) -> Result<Account> {
        ensure!(
            !private_key.is_empty(),
            errors::ERR_INVALID_PRIVATE_KEY_LENGTH
         );

        ensure!(
            private_key.len() == 64,
            errors::ERR_INVALID_KEY_LENGTH
         );

        ensure!(
            is_hex(private_key),
            errors::ERR_INVALID_KEY_HEX
            );

        let sk_hex = hex::decode(private_key)?;

        let secret_key = xpx_crypto::SecretKey::from_bytes(&sk_hex)?;

        let key_pair = xpx_crypto::Keypair::from_private_key(secret_key);

        let public_key_bytes = key_pair.public.as_bytes();

        let public_key_hex = vec_u8_to_hex(public_key_bytes.to_vec());

        let public_account = PublicAccount::from_public_key(&public_key_hex, network_type)?;

        Ok(Account {
            key_pair,
            public_account,
        })
    }

    pub fn to_private_key(&self) -> String {
        return vec_u8_to_hex(self.key_pair.secret.to_bytes().to_vec());
    }

    /// Signs 'Transaction'.
    pub fn sign(&self, tx: &dyn Transaction, generation_hash: &str) -> crate::Result<SignedTransaction> {
        tx.sign_transaction_with(self.to_owned(), generation_hash.parse().unwrap())
    }

    /// Signs raw data.
    pub fn sign_data(&self, data: &[u8]) -> String {
        let sig = &self.key_pair.sign(data).to_bytes()[..];

        return vec_u8_to_hex(sig.to_vec());
    }

    /// Creates a new encrypted message with this account as a sender.
    pub fn encrypt_message(&self) {
        todo!();
    }

    /// Decrypts an encrypted message sent for this account.
    pub fn decrypt_message(&self) {
        todo!();
    }

    /// Sign transaction with cosignatories creating a new SignedTransaction.
    pub fn sign_transaction_with_cosignatories(&self) {
        todo!();
    }

    /// Sign aggregate signature transaction.
    pub fn sign_cosignature_transaction(&self) {
        todo!();
    }
}

impl core::fmt::Display for Account {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f, "{}",
            serde_json::to_string_pretty(&self.public_account).unwrap_or_default()
        )
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountsId<'a> {
    /// The array of addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<&'a str>>,
    /// The array of public keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_keys: Option<Vec<&'a str>>
}

impl<'a>  AccountsId<'a> {
    pub(crate) fn new(addresses: Option<Vec<&'a str>>, public_keys: Option<Vec<&'a str>>) -> Self {
        AccountsId{ addresses, public_keys }
    }
}