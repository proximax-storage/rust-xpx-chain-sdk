use models::account::PublicAccount;

/// The `Account` account structure contains account's `PublicAccount` and private key.
#[derive(Debug)]
pub struct Account {
    /// The keyPair containing the public and private key of this account.
//    #[serde(rename = "keyPair")]
    pub key_pair: xpx_crypto::Keypair,
    /// The public account of this account.
//    #[serde(rename = "public_account")]
    pub public_account: PublicAccount,
}

impl Account {
    /// Create a `Account` from a private key for the given `NetworkType`.
    pub fn from_private_key(private_key: &str, network_type: crate::models::network::NetworkType) -> Result<Account, Box<dyn std::error::Error>> {
        let sk_hex = hex::decode(private_key)?;

        let secret_key = ::xpx_crypto::SecretKey::from_bytes(&sk_hex).unwrap();

        let key_pair = ::xpx_crypto::Keypair::from_private_key(secret_key);

        let public_key_bytes = key_pair.public.as_bytes();

        let public_key_hex = hex::encode(public_key_bytes);

        let public_account = PublicAccount::from_public_key(&public_key_hex, network_type);

        Ok(Account {
            key_pair,
            public_account,
        })
    }

    /// Signs raw data.
    pub fn sign_data(&self, data: &[u8]) -> String {
        let sig = &self.key_pair.sign(data).to_bytes()[..];

        return hex::encode(sig);
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
