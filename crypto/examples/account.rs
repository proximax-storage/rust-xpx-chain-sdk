extern crate base32;
extern crate hex;
extern crate ripemd160;
extern crate sha3;
extern crate xpx_crypto;

use base32::Alphabet::RFC4648;
use ripemd160::{Digest, Ripemd160};
use sha3::Sha3_256;

use xpx_crypto::PublicKey;
use xpx_crypto::SecretKey;

fn main() {
    let sk_hex = hex::decode("68f50e10e5b8be2b7e9ddb687a667d6e94d\
    d55fe02b4aed8195f51f9a242558b").unwrap();

    let secret_key: SecretKey = SecretKey::from_bytes(&sk_hex).unwrap();
    println!("PublicKey: \t{:?}", secret_key);

    println!("PrivateKey: {:?}", hex::encode(secret_key.to_bytes()));
    let public_key: PublicKey = PublicKey::from(&secret_key);

    println!("PublicKey: \t{:?}", public_key);


    let address: String = from_public_key(&public_key, 168);

    println!("{}", address);
}

pub fn from_public_key(public_key: &PublicKey, version: u8) -> String {
    let pk: Vec<u8> = public_key.to_bytes().iter().cloned().collect();

    // step 1: sha3 hash of the public key
    let sha3_public_key_hash = Sha3_256::digest(pk.as_slice());

    // step 2: Ripemd160 hash of (1)
    let ripemd160step_one_hash = Ripemd160::digest(
        sha3_public_key_hash.as_slice());

    // step 3: add version byte in front of (2)
    let version_prefixed_ripemd160hash = [&[version], &ripemd160step_one_hash[..]].concat(); // ripemd160step_one_hash.to_vec();

    // step 4: get the checksum of (3)
    let step_three_checksum = generate_checksum(&version_prefixed_ripemd160hash);

    // step 5: concatenate (3) and (4)
    let concat_step_three_and_step_six = [&version_prefixed_ripemd160hash[..],
        &step_three_checksum[..]].concat();

    let algo = base32::encode(RFC4648 { padding: true },
                              concat_step_three_and_step_six.as_slice());

    String::from(algo)
}

fn generate_checksum(vec: &Vec<u8>) -> Box<[u8]> {
    // step 1: sha3 hash of (input
    let sha3step_three_hash = Sha3_256::digest(vec.as_slice());

    // step 2: get the first numChecksumBytes bytes of (1)
    let p = &sha3step_three_hash[0..4];

    return Box::from(p.to_vec());
}
