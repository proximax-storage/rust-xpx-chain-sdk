extern crate base32;
extern crate hex;
extern crate regex;
extern crate ripemd160;
extern crate sha3;

use models::network::NetworkType;

use self::base32::Alphabet::RFC4648;
use self::regex::Regex;
use self::ripemd160::{Digest, Ripemd160};
use self::sha3::Sha3_256;

pub static HASH512_LENGTH: usize = 64;

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

    // step 2: get the first numChecksumBytes bytes of (1)
    let p = &sha3step_three_hash[0..4];

    return Box::from(p.to_vec());
}
