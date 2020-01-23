use models::account::internal::from_public_key;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Address {
    /// The address in hexadecimal.
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "networkType")]
    pub network_type: crate::models::network::NetworkType,
}

impl Address {
    pub fn from_public_key(public_key: String, network_type: crate::models::network::NetworkType) -> Address {

        let _address = from_public_key(public_key.as_str(), network_type.0);

        Address {
            address: _address,
            network_type,
        }
    }
}
