/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    anyhow::Result,
    crypto::{Keypair, SecretKey},
    crypto::Signature,
    hex::ToHex,
    rand::{rngs::ThreadRng, thread_rng},
    std::fmt,
};

use crate::{
    errors_const,
    helpers::{hex_decode, hex_encode, is_hex, Signer},
    message::{PlainMessage, SecureMessage},
    network::NetworkType,
    TransactionHash,
};
use crate::multisig::CosignatureTransaction;
use crate::transaction::{
    AggregateTransaction, CosignatureSignedTransaction, SignedTransaction, Transaction,
};

use super::{Address, PublicAccount};

/// The `Account` account structure contains account's `PublicAccount` and private key.
#[derive(Clone)]
pub struct Account {
    /// The keyPair containing the public and private key of this account.
    pub key_pair: Keypair,
    /// The public account of this account.
    pub public_account: PublicAccount,
}

impl Account {
    /// Creates an Sirius `Account` random.
    ///
    /// # Inputs
    ///
    /// * `network_type`: The `NetworkType` of Sirius Blockchain.
    ///
    /// # Example
    ///
    /// ```
    /// use xpx_chain_sdk::account::Account;
    /// use xpx_chain_sdk::network::NetworkType;
    ///
    /// #
    /// # fn main() {
    /// #
    /// let account = Account::random(NetworkType::PublicTest);
    /// # println!("{}", account);
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// A Symbol `Account`.
    pub fn random(network_type: NetworkType) -> Self {
        let mut csprng: ThreadRng = thread_rng();

        let key_pair: Keypair = Keypair::generate(&mut csprng);

        let public_key_bytes = key_pair.public.as_bytes();

        let public_key_hex = hex_encode(public_key_bytes);

        let public_account = PublicAccount::from_public_key(&public_key_hex, network_type).unwrap();

        Self { key_pair, public_account }
    }

    /// Creates an Sirius `Account` from a given hex private key string.
    ///
    /// # Inputs
    ///
    /// * `private_key`: representing the hex private key (String or &str).
    ///
    /// * `network_type`: The `NetworkType` of Sirius Blockchain.
    ///
    /// # Example
    ///
    /// ```
    /// use xpx_chain_sdk::account::Account;
    /// use xpx_chain_sdk::network::NetworkType;
    ///
    /// #
    /// # fn main() {
    /// #
    /// let private_key: &str = "75027D85CE92E2C469297F4C91E4E88AE03868A91B23C835AEF7C5EFDAD0DBDB";
    /// let account = Account::from_hex_private_key(private_key, NetworkType::PublicTest).unwrap();
    /// # println!("{}", account);
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an Sirius `Account` or whose error value
    /// is an `Error` describing the error that occurred.
    pub fn from_hex_private_key(private_key: &str, network_type: NetworkType) -> Result<Self> {
        ensure!(!private_key.is_empty(), errors_const::ERR_INVALID_PRIVATE_KEY_LENGTH);

        ensure!(private_key.len() == 64, errors_const::ERR_INVALID_KEY_LENGTH);

        ensure!(is_hex(private_key), errors_const::ERR_INVALID_KEY_HEX);

        let sk_hex = hex_decode(private_key);

        let secret_key = SecretKey::from_bytes(&sk_hex).map_err(|err| anyhow!(err))?;

        let key_pair = Keypair::from_private_key(secret_key);

        let public_key_bytes = key_pair.public.as_bytes();

        let public_key_hex = hex_encode(public_key_bytes);

        let public_account = PublicAccount::from_public_key(&public_key_hex, network_type)?;

        Ok(Self { key_pair, public_account })
    }

    pub fn key_pair_to_owned(&self) -> Keypair {
        self.key_pair.to_owned()
    }

    ///  Account private key to hex String.
    ///
    pub fn private_key_to_hex(&self) -> String {
        self.key_pair.secret.as_bytes().encode_hex::<String>()
    }

    /// Account public key to hex String.
    ///
    pub fn public_key_to_hex(&self) -> String {
        self.public_account.public_key_to_hex()
    }

    pub fn to_address(&self) -> Address {
        self.public_account.address
    }

    /// Get the `Address` in an raw address string format.
    ///
    /// For example: VAG6DUR47PZWSP7LRJSTBX26BJ5ZIROQHRJIAMHB
    pub fn address_str(&self) -> String {
        self.to_address().address_str()
    }

    ///  account `NetworkType`.
    ///
    pub fn network_type(&self) -> NetworkType {
        self.to_address().network_type
    }

    pub fn to_signer(&self) -> Signer {
        Signer::from_slice(self.public_account.to_builder())
    }

    /// Creates an encrypted message from this account to the recipient PublicAccount.
    ///
    /// # Inputs
    ///
    /// * `message`: Plain message to be encrypted.
    /// * `recipient_public_account`: Recipient public account.
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an `SecureMessage` or whose error value
    /// is an `Error` describing the error that occurred.
    pub fn encrypt_message(
        &self,
        recipient_public_account: PublicAccount,
        message: &str,
    ) -> Result<SecureMessage> {
        SecureMessage::create(self, &recipient_public_account, message)
    }

    /// Decrypts an `SecureMessage` received by this account from senderPublicAccount.
    ///
    /// # Inputs
    ///
    /// * `encrypted_message`: Encrypted message.
    /// * `signer_public_account`: The public account originally encrypted the message.
    ///
    /// A `Result` whose okay value is an `PlainMessage` or whose error value
    /// is an `Error` describing the error that occurred.
    ///
    pub fn decrypt_message(
        &self,
        sender_public_account: PublicAccount,
        secure_message: SecureMessage,
    ) -> Result<PlainMessage> {
        secure_message.decrypt(self, sender_public_account)
    }

    /// Signs raw data.
    #[inline]
    pub fn sign_data(&self, data: &str) -> Signature {
        let data = if is_hex(data) { hex::decode(data).unwrap() } else { Vec::from(data) };

        self.key_pair.sign(data.as_ref())
    }

    /// Signs 'Transaction'.
    pub fn sign_transaction(
        &self,
        tx: impl Transaction,
        generation_hash: TransactionHash,
    ) -> Result<SignedTransaction> {
        ensure!(!generation_hash.is_zero(), errors_const::ERR_EMPTY_GENERATION_HASH);

        tx.sign_with(self.to_owned(), generation_hash)
    }

    // /// Sign transaction with cosignatories creating a new signed_transaction.
    pub fn sign_with_cosignatories(
        &self,
        tx: AggregateTransaction,
        cosignatories: Vec<Account>,
        generation_hash: TransactionHash,
    ) -> Result<SignedTransaction> {
        ensure!(!generation_hash.is_zero(), errors_const::ERR_EMPTY_GENERATION_HASH);

        tx.sign_with_cosignatories(self.to_owned(), cosignatories, generation_hash)
    }

    /// Sign aggregate signature transaction.
    pub fn sign_cosignature_transaction(
        &self,
        tx: CosignatureTransaction,
    ) -> Result<CosignatureSignedTransaction> {
        tx.sign_cosignature_transaction(self.to_owned())
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self.public_account).unwrap_or_default())
    }
}

impl fmt::Debug for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Account").field(&self.public_account).finish()
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
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
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
        Self::from(ids.into_iter().map(|id| id.to_owned()).collect::<Vec<_>>())
    }
}

impl From<Vec<String>> for AccountsId {
    #[inline]
    fn from(ids: Vec<String>) -> Self {
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

#[cfg(test)]
pub mod tests {
    use lazy_static::lazy_static;

    use crate::account::Account;
    use crate::network::NetworkType;

    lazy_static! {
		pub static ref TESTING_ACCOUNT: Account = Account::from_hex_private_key(
			"26b64cb10f005e5988a36744ca19e20d835ccc7c105aaa5f3b212da593180930",
			NetworkType::PrivateTest
		)
		.unwrap();
		pub static ref MULTISIG_ACCOUNT: Account = Account::from_hex_private_key(
			"5edebfdbeb32e9146d05ffd232c8af2cf9f396caf9954289daa0362d097fff3b",
			NetworkType::PrivateTest
		)
		.unwrap();
		pub static ref COSIGNATORY_ACCOUNT: Account = Account::from_hex_private_key(
			"2a2b1f5d366a5dd5dc56c3c757cf4fe6c66e2787087692cf329d7a49a594658b",
			NetworkType::PrivateTest
		)
		.unwrap();
		pub static ref COSIGNATORY2_ACCOUNT: Account = Account::from_hex_private_key(
			"b8afae6f4ad13a1b8aad047b488e0738a437c7389d4ff30c359ac068910c1d59",
			NetworkType::PrivateTest
		)
		.unwrap();
		pub static ref COSIGNATORY3_ACCOUNT: Account = Account::from_hex_private_key(
			"111602be4d36f92dd60ca6a3c68478988578f26f6a02f8c72089839515ab603e",
			NetworkType::PrivateTest
		)
		.unwrap();
	}

    #[cfg(test)]
    mod tests_account {
        use super::*;

        const ADDRESS: &str = "WCTVW23D2MN5VE4AQ4TZIDZENGNOZXPRPSIBCI5Q";
        const PUBLIC_KEY: &str = "c2f93346e27ce6ad1a9f8f5e3066f8326593a406bdf357acb041e2f9ab402efe";
        const PRIVATE_KEY: &str =
            "26b64cb10f005e5988a36744ca19e20d835ccc7c105aaa5f3b212da593180930";

        #[test]
        fn test_should_create_from_private_key() {
            let account =
                Account::from_hex_private_key(PRIVATE_KEY, NetworkType::PrivateTest).unwrap();
            assert_eq!(account.public_key_to_hex(), PUBLIC_KEY.to_lowercase());
            assert_eq!(account.private_key_to_hex(), PRIVATE_KEY.to_lowercase());

            assert_eq!(account.address_str(), ADDRESS);
        }

        #[test]
        fn test_should_return_error_when_the_private_key_is_not_valid() {
            let account = Account::from_hex_private_key("", NetworkType::PrivateTest);
            assert!(account.is_err());
        }

        #[test]
        fn test_should_generate_a_new_account() {
            let account = Account::random(NetworkType::PrivateTest);

            assert_ne!(account.private_key_to_hex(), "");
            assert_ne!(account.public_key_to_hex(), "");
            assert_ne!(account.address_str(), "");
        }

        #[test]
        fn test_should_return_network_type() {
            let account = Account::random(NetworkType::PublicTest);
            assert_eq!(account.network_type(), NetworkType::PublicTest);
        }
    }

    #[cfg(test)]
    mod tests_sign_data {
        use super::*;

        const PRIVATE_KEY: &str =
            "AB860ED1FE7C91C02F79C02225DAC708D7BD13369877C1F59E678CC587658C47";

        #[test]
        fn test_utf8_data() {
            let account =
                Account::from_hex_private_key(PRIVATE_KEY, NetworkType::PrivateTest).unwrap();

            let data = "proximax is wonderful!";

            let public_account = account.public_account;
            let signed = account.sign_data(data);
            assert!(public_account.verify_signature(data, signed).is_ok());
        }

        #[test]
        fn test_hex_data() {
            let account =
                Account::from_hex_private_key(PRIVATE_KEY, NetworkType::PrivateTest).unwrap();

            let data = "0xAA";

            let public_account = account.public_account;
            let signed = account.sign_data(data);
            assert!(public_account.verify_signature(data, signed).is_ok());
        }

        #[test]
        fn test_hexa_without_0x_prefix_should_be_the_same_as_with_0x() {
            let account =
                Account::from_hex_private_key(PRIVATE_KEY, NetworkType::PrivateTest).unwrap();

            let public_account = account.public_account;

            let signed = account.sign_data("AA");
            let signed_with0x = account.sign_data("0xAA");

            assert!(public_account.verify_signature("AA", signed).is_ok());
            assert!(public_account.verify_signature("0xAA", signed_with0x).is_ok());
        }

        #[test]
        fn test_sign_empty() {
            let account =
                Account::from_hex_private_key(PRIVATE_KEY, NetworkType::PrivateTest).unwrap();

            let public_account = account.public_account;

            let signed = account.sign_data("");
            let signed_with0x = account.sign_data("0x");

            assert!(public_account.verify_signature("", signed).is_ok());
            assert!(public_account.verify_signature("0x", signed_with0x).is_ok());
        }
    }
}
