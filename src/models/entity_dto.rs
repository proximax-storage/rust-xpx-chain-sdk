#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityDto {
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: u16,
}
