// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use {
    crypto::{Keypair, SecretKey},
    rand::rngs::OsRng,
    utils::{is_hex, vec_u8_to_hex},
};

use crate::{
    models::{
        errors_const,
        multisig::CosignatureTransaction,
        network::NetworkType,
        transaction::{
            AggregateTransaction, CosignatureSignedTransaction, SignedTransaction, Transaction,
        },
    },
    Result,
};

use super::{Address, PublicAccount};

pub type AccountId = String;

/// The `Account` account structure contains account's `PublicAccount` and private key.
#[derive(Debug, Clone)]
pub struct Account {
    /// The keyPair containing the public and private key of this account.
    pub key_pair: Keypair,
    /// The public account of this account.
    pub public_account: PublicAccount,
}

impl Account {
    pub fn new(network_type: NetworkType) -> Self {
        let mut csprng: OsRng = OsRng::new().unwrap();
        let key_pair: Keypair = Keypair::generate(&mut csprng);

        let public_key_bytes = key_pair.public.as_bytes();

        let public_key_hex = vec_u8_to_hex(public_key_bytes.to_vec());

        let public_account = PublicAccount::from_public_key(&public_key_hex, network_type).unwrap();

        Self {
            key_pair,
            public_account,
        }
    }

    pub fn key_pair_to_owned(&self) -> Keypair {
        self.key_pair.to_owned()
    }

    pub fn public_account_to_owned(&self) -> PublicAccount {
        self.to_owned().public_account
    }

    pub fn public_key_string(&self) -> String {
        self.public_account_to_owned().public_key_string()
    }

    pub fn address(&self) -> Address {
        self.public_account.address_to_owned()
    }

    pub fn address_string(&self) -> String {
        self.address().address_string()
    }

    /// Create a `Account` from a private key for the given `NetworkType`.
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

        let sk_hex = hex::decode(private_key)?;

        let secret_key = SecretKey::from_bytes(&sk_hex)?;

        let key_pair = Keypair::from_private_key(secret_key);

        let public_key_bytes = key_pair.public.as_bytes();

        let public_key_hex = vec_u8_to_hex(public_key_bytes.to_vec());

        let public_account = PublicAccount::from_public_key(&public_key_hex, network_type)?;

        Ok(Self {
            key_pair,
            public_account,
        })
    }

    pub fn to_private_key(&self) -> String {
        vec_u8_to_hex(self.key_pair.secret.to_bytes().to_vec())
    }

    /// Signs 'Transaction'.
    pub fn sign(
        &self,
        tx: impl Transaction,
        generation_hash: &str,
    ) -> crate::Result<SignedTransaction> {
        ensure!(
            !generation_hash.is_empty(),
            errors_const::ERR_EMPTY_GENERATION_HASH
        );

        tx.sign_transaction_with(self.to_owned(), generation_hash.parse()?)
    }

    /// Signs raw data.
    #[inline]
    pub fn sign_data(&self, data: &[u8]) -> String {
        let sig = &self.key_pair.sign(data).to_bytes()[..];

        vec_u8_to_hex(sig.to_vec())
    }

    /// Creates a new encrypted message with this account as a sender.
    pub fn encrypt_message(&self) {
        todo!();
    }

    /// Decrypts an encrypted message sent for this account.
    pub fn decrypt_message(&self) {
        todo!();
    }

    /// Sign transaction with cosignatories creating a new signed_transaction.
    pub fn sign_with_cosignatories(
        &self,
        tx: AggregateTransaction,
        cosignatories: Vec<Account>,
        generation_hash: &str,
    ) -> crate::Result<SignedTransaction> {
        ensure!(
            !generation_hash.is_empty(),
            errors_const::ERR_EMPTY_GENERATION_HASH
        );

        tx.sign_with_cosignatories(self.to_owned(), cosignatories, generation_hash.parse()?)
    }

    /// Sign aggregate signature transaction.
    pub fn sign_cosignature_transaction(
        &self,
        tx: CosignatureTransaction,
    ) -> crate::Result<CosignatureSignedTransaction> {
        tx.sign_cosignature_transaction(self.to_owned())
    }
}

impl core::fmt::Display for Account {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self.public_account).unwrap_or_default()
        )
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountName {
    pub address: Address,
    pub names: Vec<String>,
}

impl core::fmt::Display for AccountName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
