/// EntityType The entity type:
/// * 0x4158 (16728 decimal) - Blockchain Upgrade Transaction.
/// * 0x4159 (16729 decimal) - Network Config Transaction.
/// * 0x413D (16701 decimal) - Address Metadata Transaction.
/// * 0x423D (16957 decimal) - Mosaic Metadata Transaction.
/// * 0x433D (17213 decimal) - Namespace Metadata Transaction.
/// * 0x414D (16717 decimal) - Mosaic Definition Transaction.
/// * 0x424D (16973 decimal) - Mosaic Supply Change Transaction.
/// * 0x414E (16718 decimal) - Register Namespace Transaction.
/// * 0x424E (16974 decimal) - Address Alias Transaction.
/// * 0x434E (17230 decimal) - Mosaic Alias Transaction.
/// * 0x4154 (16724 decimal) - Transfer Transaction.
/// * 0x4155 (16725 decimal) - Modify Multisig Account Transaction.
/// * 0x4141 (16705 decimal) - Aggregate Complete Transaction.
/// * 0x4241 (16961 decimal) - Aggregate Bonded Transaction.
/// * 0x4148 (16712 decimal) - Hash Lock Transaction.
/// * 0x4150 (16720 decimal) - Account Properties Address Transaction.
/// * 0x4250 (16976 decimal) - Account Properties Mosaic Transaction.
/// * 0x4350 (17232 decimal) - Account Properties Entity Type Transaction.
/// * 0x4152 (16722 decimal) - Secret Lock Transaction.
/// * 0x4252 (16978 decimal) - Secret Proof Transaction.
/// * 0x414C (16716 decimal) - Account Link Transaction.
/// * 0x8043 (32835 decimal) - Nemesis block.
/// * 0x8143 (33091 decimal) - Regular block.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[repr(u16)]
pub enum EntityType {
    #[serde(rename = "16728")]
    _16728 = 0x4158,
    #[serde(rename = "16729")]
    _16729 = 0x4159,
    #[serde(rename = "16701")]
    _16701 = 0x413D,
    #[serde(rename = "16957")]
    _16957 = 0x423D,
    #[serde(rename = "17213")]
    _17213 = 0x433D,
    #[serde(rename = "16717")]
    _16717 = 0x414D,
    #[serde(rename = "16973")]
    _16973 = 0x424D,
    #[serde(rename = "16718")]
    _16718 = 0x414E,
    #[serde(rename = "16974")]
    _16974 = 0x424E,
    #[serde(rename = "17230")]
    _17230 = 0x434E,
    #[serde(rename = "16724")]
    _16724 = 0x4154,
    #[serde(rename = "16725")]
    _16725 = 0x4155,
    #[serde(rename = "16705")]
    _16705 = 0x4141,
    #[serde(rename = "16961")]
    _16961 = 0x4241,
    #[serde(rename = "16712")]
    _16712 = 0x4148,
    #[serde(rename = "16720")]
    _16720 = 0x4150,
    #[serde(rename = "16976")]
    _16976 = 0x4250,
    #[serde(rename = "17232")]
    _17232 = 0x4350,
    #[serde(rename = "16722")]
    _16722 = 0x4152,
    #[serde(rename = "16978")]
    _16978 = 0x4252,
    #[serde(rename = "16716")]
    _16716 = 0x414C,
    #[serde(rename = "32835")]
    _32835 = 0x8043,
    #[serde(rename = "33091")]
    _33091 = 0x8143,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityDto {
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - PUBLIC main network. * 0x98 (TEST_NET) - PUBLIC test network. * 0x60 (MIJIN) - PRIVATE network. * 0x90 (MIJIN_TEST) - PRIVATE test network.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: EntityType,
}

impl EntityDto {
    pub fn new(signer: String, version: i32, _type: EntityType) -> EntityDto {
        EntityDto {
            signer,
            version,
            _type,
        }
    }
}


