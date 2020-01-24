use models::account::Address;

/// The `PublicAccount` account structure contains account's `Address` and public key.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicAccount {
    /// Retrieves the `Address` of this public account.
    #[serde(rename = "address")]
    pub address: Address,
    /// Retrieves the public key of this public account.
    #[serde(rename = "publicKey")]
    pub public_key: String,
}

impl PublicAccount {
    /// Create a `PublicAccount` from a public key for the given `NetworkType`.
    pub fn from_public_key(public_key: &str, network_type: crate::models::network::NetworkType) -> PublicAccount {
        let _address = ::models::account::public_key_to_address(public_key, network_type.0);

        PublicAccount {
            address: Address::from_public_key(public_key, network_type),
            public_key: public_key.to_string(),
        }
    }
}
