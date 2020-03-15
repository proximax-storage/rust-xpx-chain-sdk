use ::sha3::Sha3_256;
use fb::FlatBufferBuilder;
use sha3::Digest;
use xpx_crypto::Keypair;

use crate::models::{
    account::Account,
    consts::{HALF_OF_SIGNATURE, SIGNATURE_SIZE, SIGNER_SIZE, SIZE_SIZE},
    mosaic::MosaicProperty,
    multisig::CosignatoryModification,
    utils::vec_u8_to_hex,
};
use crate::models::consts::{TRANSACTION_HEADER_SIZE, TYPE_SIZE, VERSION_SIZE};
use crate::models::errors::ERR_EMPTY_TRANSACTION_SIGNER;
use crate::models::utils::u32_to_array_u8;

use super::{EntityVersion, SignedTransaction, Transaction};
use super::buffer::{
    modify_multisig_account,
    mosaic_definition
};

pub(crate) fn extract_version(version: i32) -> EntityVersion {
    return version & 0xFFFFFF;
}

pub(crate) fn sign_transaction(tx: impl Transaction, account: Account, generation_hash: String) -> crate::Result<SignedTransaction> {
    let key_pair: Keypair = Keypair::from_private_key(account.key_pair.secret);

    let mut bytes = tx.embedded_to_bytes();

    let generation_hash_bytes = hex::decode(&generation_hash);

    let signing_suffix = &bytes[SIZE_SIZE + SIGNER_SIZE + SIGNATURE_SIZE..];

    let signing = [generation_hash_bytes.unwrap().as_slice(), signing_suffix].concat();

    let signature = key_pair.sign(&signing);

    let mut tx_vector: Vec<u8> = Vec::with_capacity(bytes.len());

    tx_vector.append(&mut bytes[..4].to_vec());
    tx_vector.append(&mut signature.to_bytes().to_vec());
    tx_vector.append(&mut key_pair.public.to_bytes().to_vec());
    tx_vector.append(&mut bytes[SIZE_SIZE + SIGNER_SIZE + SIGNATURE_SIZE..].to_vec());

    let p_hex = vec_u8_to_hex(tx_vector);

    let hash = create_transaction_hash(p_hex.clone(), &generation_hash);

    Ok(SignedTransaction::new(tx.entity_type(), p_hex, hash))
}

pub(crate) fn create_transaction_hash(p: String, generation_hash: &str) -> String {
    let mut p_bytes = hex::decode(p).unwrap();

    let mut sb = Vec::new();

    sb.append(&mut p_bytes[SIZE_SIZE..SIZE_SIZE + HALF_OF_SIGNATURE].to_vec());

    sb.append(&mut p_bytes[SIGNATURE_SIZE + SIZE_SIZE..SIZE_SIZE + SIGNATURE_SIZE + SIGNER_SIZE].to_vec());

    let generation_hash_bytes = hex::decode(generation_hash);

    sb.append(&mut generation_hash_bytes.unwrap().to_vec());

    sb.append(&mut p_bytes[100..].to_vec());

    let sha3_public_key_hash = Sha3_256::digest(sb.as_slice());

    vec_u8_to_hex(sha3_public_key_hash[..].to_vec())
}

pub(crate) fn mosaic_property_array_to_buffer(
    builder: &mut FlatBufferBuilder, properties: Vec<MosaicProperty>) -> fb::UOffsetT {
    let mut p_buffer: Vec<fb::UOffsetT> = Vec::with_capacity(properties.len());

    for p in properties {
        let value_v = builder.create_vector(&p.value.to_int_array());

        let mut mosaic_property = mosaic_definition::buffers::MosaicPropertyBuilder::new(builder);
        mosaic_property.add_mosaic_property_id(p.id.value());
        mosaic_property.add_value(value_v);

        p_buffer.push(mosaic_property.finish().value());
    }

    builder.create_vector(&p_buffer).value()
}

pub(crate) fn cosignatory_modification_array_to_buffer(
    builder: &mut FlatBufferBuilder, modifications: Vec<CosignatoryModification>) -> fb::UOffsetT {
    let mut p_buffer: Vec<fb::UOffsetT> = Vec::with_capacity(modifications.len());

    for modification in modifications {
        let public_key = modification.public_account;
        let public_key_vector = builder.create_vector(&public_key.to_array());

        let mut modify_multisig = modify_multisig_account::buffers::CosignatoryModificationBufferBuilder::new(builder);
        modify_multisig.add_type_(modification.modification_type.value());
        modify_multisig.add_cosignatory_public_key(public_key_vector);
        p_buffer.push(modify_multisig.finish().value());
    }

    builder.create_vector(&p_buffer).value()
}

pub(crate) fn to_aggregate_transaction_bytes(tx: &Box<dyn Transaction>) -> crate::Result<Vec<u8>> {
    ensure!(
        tx.abs_transaction().signer.public_key != "",
        ERR_EMPTY_TRANSACTION_SIGNER
    );

    let mut signer_bytes = tx.abs_transaction().signer.to_array();

    let mut tx_vec = tx.embedded_to_bytes();

    let mut r_b: Vec<u8> = Vec::with_capacity(tx_vec.len());
    r_b.append(&mut [0, 0, 0, 0].to_vec());
    r_b.append(&mut signer_bytes[..].to_vec());
    r_b.append(&mut tx_vec[
        SIZE_SIZE + SIGNER_SIZE + SIGNATURE_SIZE..SIZE_SIZE + SIGNER_SIZE + SIGNATURE_SIZE + VERSION_SIZE
            + TYPE_SIZE].to_vec());

    r_b.append(&mut tx_vec[TRANSACTION_HEADER_SIZE..].to_vec());

    let s = u32_to_array_u8(r_b.len() as u32);

    for (i, b) in s.iter().enumerate() {
        r_b[i] = *b;
    };

    Ok(r_b)
}