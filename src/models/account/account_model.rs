/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    crypto::{Keypair, SecretKey},
    rand::{rngs::ThreadRng, thread_rng},
    std::fmt,
};

use crate::{
    helpers::{hex_decode, hex_encode, is_hex},
    models::{
        errors_const,
        message::{PlainMessage, SecureMessage},
        multisig::CosignatureTransaction,
        network::NetworkType,
        transaction::{
            AggregateTransaction, CosignatureSignedTransaction, HashValue, SignedTransaction,
            Signer, Transaction,
        },
    },
    Result,
};

use super::{Address, PublicAccount};

pub type AccountId = String;

/// The `Account` account structure contains account's `PublicAccount` and private key.
#[derive(Clone)]
pub struct Account {
    /// The keyPair containing the public and private key of this account.
    pub key_pair: Keypair,
    /// The public account of this account.
    pub public_account: PublicAccount,
}

impl Account {
    /// Generates a new [`Account`] random for the given [`NetworkType`].
    pub fn new(network_type: NetworkType) -> Self {
        let mut csprng: ThreadRng = thread_rng();

        let key_pair: Keypair = Keypair::generate(&mut csprng);

        let public_key_bytes = key_pair.public.as_bytes();

        let public_key_hex = hex_encode(public_key_bytes);

        let public_account = PublicAccount::from_public_key(&public_key_hex, network_type).unwrap();

        Self {
            key_pair,
            public_account,
        }
    }

    /// Create a [`Account`] from a private key for the given [`NetworkType`].
    pub fn from_private_key(private_key: &str, network_type: NetworkType) -> Result<Self> {
        ensure!(
            !private_key.is_empty(),
            errors_const::ERR_INVALID_PRIVATE_KEY_LENGTH
        );

        ensure!(
            private_key.len() == 64,
            errors_const::ERR_INVALID_KEY_LENGTH
        );

        ensure!(is_hex(private_key), errors_const::ERR_INVALID_KEY_HEX);

        let sk_hex = hex_decode(private_key);

        let secret_key = SecretKey::from_bytes(&sk_hex)?;

        let key_pair = Keypair::from_private_key(secret_key);

        let public_key_bytes = key_pair.public.as_bytes();

        let public_key_hex = hex_encode(public_key_bytes);

        let public_account = PublicAccount::from_public_key(&public_key_hex, network_type)?;

        Ok(Self {
            key_pair,
            public_account,
        })
    }

    pub fn key_pair_to_owned(&self) -> Keypair {
        self.key_pair.to_owned()
    }

    /// The public key string of this account.
    pub fn public_key_string(&self) -> String {
        self.public_account.public_key_string()
    }

    pub fn to_address(&self) -> Address {
        self.public_account.address
    }

    /// The plain text address of this account.
    pub fn address_string(&self) -> String {
        self.to_address().address_string()
    }

    /// The private key hex string of this account.
    pub fn to_private_key(&self) -> String {
        hex_encode(&self.key_pair.secret.to_bytes())
    }

    /// The network type of this account.
    pub fn network_type(&self) -> NetworkType {
        self.to_address().network_type()
    }

    pub fn to_signer(&self) -> Signer {
        Signer::from_slice(self.public_account.to_bytes()).unwrap()
    }

    /// Creates an encrypted message from this account to the [recipientPublicAccount].
    pub fn encrypt_message(
        &self,
        recipient_public_account: &PublicAccount,
        message: &str,
    ) -> Result<SecureMessage> {
        SecureMessage::create(self, &recipient_public_account, message)
    }

    /// Decrypts an encrypted message received by this account from [senderPublicAccount].
    pub fn decrypt_message(
        &self,
        sender_public_account: &PublicAccount,
        secure_message: SecureMessage,
    ) -> Result<PlainMessage> {
        secure_message.decrypt(self, sender_public_account)
    }

    /// Signs 'Transaction'.
    pub fn sign(
        &self,
        tx: impl Transaction,
        generation_hash: HashValue,
    ) -> crate::Result<SignedTransaction> {
        ensure!(
            !generation_hash.is_empty(),
            errors_const::ERR_EMPTY_GENERATION_HASH
        );

        tx.sign_transaction_with(self.to_owned(), generation_hash)
    }

    /// Signs raw data.
    #[inline]
    pub fn sign_data(&self, data: &[u8]) -> String {
        let sig = &self.key_pair.sign(data).to_bytes()[..];

        hex_encode(sig)
    }

    /// Sign transaction with cosignatories creating a new signed_transaction.
    pub fn sign_with_cosignatories(
        &self,
        tx: AggregateTransaction,
        cosignatories: Vec<Account>,
        generation_hash: HashValue,
    ) -> crate::Result<SignedTransaction> {
        ensure!(
            !generation_hash.is_empty(),
            errors_const::ERR_EMPTY_GENERATION_HASH
        );

        tx.sign_with_cosignatories(self.to_owned(), cosignatories, generation_hash)
    }

    /// Sign aggregate signature transaction.
    pub fn sign_cosignature_transaction(
        &self,
        tx: CosignatureTransaction,
    ) -> crate::Result<CosignatureSignedTransaction> {
        tx.sign_cosignature_transaction(self.to_owned())
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self.public_account).unwrap_or_default()
        )
    }
}

impl fmt::Debug for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Account")
            .field(&self.public_account)
            .finish()
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountName {
    pub address: Address,
    pub names: Vec<String>,
}

impl fmt::Display for AccountName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountsId {
    /// The array of addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    /// The array of public keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_keys: Option<Vec<String>>,
}

impl From<Vec<&str>> for AccountsId {
    #[inline]
    fn from(ids: Vec<&str>) -> Self {
        let mut public_keys = vec![];
        let mut addresses = vec![];

        let mut accounts = AccountsId::default();

        for (i, id) in ids.iter().enumerate() {
            let _id = id.trim();

            if is_hex(_id) && _id.len() == 64 {
                public_keys.push(_id.to_uppercase());
            } else {
                addresses.push(_id.replace("-", "").to_uppercase());
            }

            if i == ids.len() - 1 {
                if !public_keys.is_empty() {
                    accounts.public_keys = Some(public_keys.to_owned())
                } else if !addresses.is_empty() {
                    accounts.addresses = Some(addresses.to_owned())
                }
            }
        }
        accounts
    }
}
