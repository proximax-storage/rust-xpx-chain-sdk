use ::base32::Alphabet::RFC4648;
use ::ripemd160::{Digest, Ripemd160};
use ::sha3::Sha3_256;

use crate::models::network::NetworkType;

pub static HASH512_LENGTH: usize = 64;

pub static EMPTY_PUBLIC_KEY: &str = "0000000000000000000000000000000000000000000000000000000000000000";

/// AccountPropertyTypeEnum :
/// The account properties type:
/// * 0x01 (1 decimal) - The property type only allows receiving transactions from an address.
/// * 0x02 (2 decimal) - The property type only allows receiving transactions containing a mosaic id.
/// * 0x04 (4 decimal) - The property type only allows sending transactions with a given transaction type.
/// * 0x05 (5 decimal) - Property type sentinel.
/// * 0x81 (129 decimal) - The property type blocks receiving transactions from an address.
/// * 0x82 (130 decimal) - The property type blocks receiving transactions containing a mosaic id.
/// * 0x84 (132 decimal) -  The property type blocks sending transactions with a given transaction type.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AccountPropertyTypeEnum {
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,
    #[serde(rename = "4")]
    _4,
    #[serde(rename = "5")]
    _5,
    #[serde(rename = "129")]
    _129,
    #[serde(rename = "130")]
    _130,
    #[serde(rename = "132")]
    _132,

}

/// AccountPropertiesModificationTypeEnum :
/// The account properties modification type: * 0 - Add property. * 1 - Remove property.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AccountPropertiesModificationTypeEnum {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,

}

pub fn public_key_to_address(public_key: &str, version: NetworkType) -> String {
    let pk: Vec<u8> = hex::decode(public_key).unwrap();

    // step 1: sha3 hash of the public key
    let sha3_public_key_hash = Sha3_256::digest(pk.as_slice());

    // step 2: Ripemd160 hash of (1)
    let ripemd160step_one_hash = Ripemd160::digest(
        sha3_public_key_hash.as_slice());

    // step 3: add version byte in front of (2)
    let version_prefixed_ripemd160hash = [&[version.0], &ripemd160step_one_hash[..]].concat(); // ripemd160step_one_hash.to_vec();

    // step 4: get the checksum of (3)
    let step_three_checksum = generate_checksum(&version_prefixed_ripemd160hash);

    // step 5: concatenate (3) and (4)
    let concat_step_three_and_step_six = [&version_prefixed_ripemd160hash[..],
        &step_three_checksum[..]].concat();

    let res = base32::encode(RFC4648 { padding: true },
                             concat_step_three_and_step_six.as_slice());

    String::from(res)
}

fn generate_checksum(vec: &Vec<u8>) -> Box<[u8]> {
    // step 1: sha3 hash of (input
    let sha3step_three_hash = Sha3_256::digest(vec.as_slice());

    // step 2: get the first NUM_CHECKSUM_BYTES bytes of (1)
    let p = &sha3step_three_hash[0..4];

    return Box::from(p.to_vec());
}
