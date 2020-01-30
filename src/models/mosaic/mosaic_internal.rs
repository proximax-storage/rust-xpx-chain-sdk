extern crate byteorder;
extern crate sha3;

use models::account::PublicAccount;
use models::mosaic::MosaicNonce;
use models::namespace::NAMESPACE_BIT;
use models::Uint64;
use models::utils::array_u8_to_u64;

use self::sha3::{Digest, Sha3_256};

pub(crate) fn generate_mosaic_id(nonce: MosaicNonce, owner_public_id: PublicAccount) -> Uint64 {
    let mut hash = Sha3_256::default();

    let nonce_bytes = nonce.to_array();

    hash.input(nonce_bytes);

    let owner_bytes: [u8; 32] = owner_public_id.to_array();

    hash.input(owner_bytes);

    let hash_to_array = hash.result();

    Uint64(array_u8_to_u64(&mut hash_to_array.as_slice()) ^ NAMESPACE_BIT)
}
