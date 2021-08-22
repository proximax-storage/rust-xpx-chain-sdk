/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {::sha3::Sha3_256, ::std::str::FromStr, fb::FlatBufferBuilder, sha3::Digest};

use crate::{
    helpers::{hex_decode, hex_encode, u32_to_array_u8},
    models::{
        account::Account,
        consts::{
            HALF_OF_SIGNATURE, SIGNATURE_SIZE, SIGNER_SIZE, SIZE_SIZE, TRANSACTION_HEADER_SIZE,
            TYPE_SIZE, VERSION_SIZE,
        },
        errors_const::ERR_EMPTY_TRANSACTION_SIGNER,
        metadata::MetadataModification,
        mosaic::MosaicProperty,
        multisig::CosignatoryModification,
    },
};

use super::{
    AbsTransaction,
    AggregateTransaction, buffer::{modify_metadata, modify_multisig_account as modify_multisig, mosaic_definition}, EntityVersion, HashValue, SignedTransaction, Transaction,
};

pub(crate) fn extract_version(version: u32) -> EntityVersion {
    version & 0xFFFFFF
}

pub(crate) fn sign_transaction(
    tx: impl Transaction,
    account: Account,
    generation_hash: HashValue,
) -> crate::Result<SignedTransaction> {
    let key_pair: crypto::Keypair = crypto::Keypair::from_private_key(account.key_pair.secret);

    let tx_bytes = tx.embedded_to_bytes()?;

    let generation_hash_bytes = generation_hash.as_bytes();

    let signing_suffix = &tx_bytes[SIZE_SIZE + SIGNER_SIZE + SIGNATURE_SIZE..];

    let signing = [generation_hash_bytes, signing_suffix].concat();

    let signature = key_pair.sign(&signing);

    let mut tx_vector: Vec<u8> = Vec::with_capacity(tx_bytes.len());

    tx_vector.extend_from_slice(&tx_bytes[..4]);
    tx_vector.extend_from_slice(&signature.to_bytes());
    tx_vector.extend_from_slice(&key_pair.public.to_bytes());
    tx_vector.extend_from_slice(&tx_bytes[SIZE_SIZE + SIGNER_SIZE + SIGNATURE_SIZE..]);

    let payload = hex_encode(&tx_vector);

    let hash = create_transaction_hash(payload.clone(), generation_hash);

    Ok(SignedTransaction::new(
        tx.entity_type(),
        payload,
        HashValue::from_str(&hash)?,
    ))
}

pub(crate) fn sign_transaction_with_cosignatures(
    tx: AggregateTransaction,
    account: Account,
    cosignatories: Vec<Account>,
    generation_hash: HashValue,
) -> crate::Result<SignedTransaction> {
    let entity_type = tx.entity_type();
    let stx = sign_transaction(tx, account, generation_hash)?;

    let mut payload = stx.to_owned().payload.unwrap();
    cosignatories.iter().for_each(|item| {
        let key_pair: crypto::Keypair =
            crypto::Keypair::from_private_key(item.to_owned().key_pair.secret);
        let hash_bytes = stx.hash.to_vec();
        let signature = key_pair.sign(&hash_bytes);
        payload.push_str(&format!(
            "{}{}",
            item.public_key_string(),
            hex_encode(&signature.to_bytes()[..])
        ));
    });

    let mut payload_bytes = hex_decode(&payload);

    let s = u32_to_array_u8(payload_bytes.len() as u32);

    s.iter().enumerate().for_each(|(i, item)| {
        payload_bytes[i] = *item;
    });

    Ok(SignedTransaction::new(
        entity_type,
        hex_encode(&payload_bytes),
        stx.hash,
    ))
}

pub(crate) fn create_transaction_hash(p: String, generation_hash: HashValue) -> String {
    let p_bytes = hex_decode(&p);

    let mut sb = vec![];

    sb.extend_from_slice(&p_bytes[SIZE_SIZE..SIZE_SIZE + HALF_OF_SIGNATURE]);

    sb.extend_from_slice(
        &p_bytes[SIGNATURE_SIZE + SIZE_SIZE..SIZE_SIZE + SIGNATURE_SIZE + SIGNER_SIZE],
    );

    let generation_hash_bytes = generation_hash.as_bytes();

    sb.extend_from_slice(generation_hash_bytes);

    sb.extend_from_slice(&p_bytes[100..]);

    let sha3_public_key_hash = Sha3_256::digest(sb.as_slice());

    hex_encode(&sha3_public_key_hash[..])
}

pub(crate) fn mosaic_property_array_to_buffer(
    builder: &mut FlatBufferBuilder,
    properties: Vec<MosaicProperty>,
) -> fb::UOffsetT {
    let mut p_buffer: Vec<fb::UOffsetT> = Vec::with_capacity(properties.len());

    for p in properties {
        let value_v = builder.create_vector(&p.value.to_u32_array());

        let mut mosaic_property = mosaic_definition::MosaicPropertyBuilder::new(builder);
        mosaic_property.add_mosaic_property_id(p.id.value());
        mosaic_property.add_value(value_v);

        p_buffer.push(mosaic_property.finish().value());
    }

    builder.create_vector(&p_buffer).value()
}

pub(crate) fn to_aggregate_transaction_bytes(tx: &Box<dyn Transaction>) -> crate::Result<Vec<u8>> {
    ensure!(
        tx.abs_transaction().signer.public_key_string() != "",
        ERR_EMPTY_TRANSACTION_SIGNER
    );

    let signer_bytes = tx.to_owned().abs_transaction().signer.public_key;

    let tx_bytes = tx.embedded_to_bytes()?;

    let mut r_b: Vec<u8> = Vec::with_capacity(tx_bytes.len());
    r_b.extend_from_slice(&[0u8; 4]);
    r_b.extend_from_slice(&signer_bytes[..]);
    r_b.extend_from_slice(
        &tx_bytes[SIZE_SIZE + SIGNER_SIZE + SIGNATURE_SIZE
            ..SIZE_SIZE + SIGNER_SIZE + SIGNATURE_SIZE + VERSION_SIZE + TYPE_SIZE],
    );

    r_b.extend_from_slice(&tx_bytes[TRANSACTION_HEADER_SIZE..]);

    let s = u32_to_array_u8(r_b.len() as u32);

    for (i, item) in s.iter().enumerate() {
        r_b[i] = *item;
    }

    Ok(r_b)
}

pub(crate) fn cosignatory_modification_array_to_buffer<'a>(
    builder: &mut FlatBufferBuilder<'a>,
    modifications: Vec<CosignatoryModification>,
) -> fb::UOffsetT {
    let mut cosignatory_buffer: Vec<
        fb::WIPOffset<modify_multisig::CosignatoryModificationBuffer<'a>>,
    > = Vec::with_capacity(modifications.len());

    for modification in modifications {
        let public_key = &modification.public_account;

        let public_key_vector = builder.create_vector_direct(&public_key.to_bytes());

        let mut modify_multisig =
            modify_multisig::CosignatoryModificationBufferBuilder::new(builder);
        modify_multisig.add_type_(modification.modification_type.value());
        modify_multisig.add_cosignatory_public_key(public_key_vector);

        cosignatory_buffer.push(modify_multisig.finish());
    }

    builder.create_vector(&cosignatory_buffer).value()
}

pub(crate) fn metadata_modification_array_to_buffer<'a>(
    builder: &mut FlatBufferBuilder<'a>,
    modifications: Vec<MetadataModification>,
) -> fb::UOffsetT {
    let mut modification_buffer: Vec<
        fb::WIPOffset<modify_metadata::MetadataModificationBuffer<'a>>,
    > = Vec::with_capacity(modifications.len());

    for modification in modifications {
        let key_size = modification.key.len();
        if key_size == 0 {
            return 0;
        }

        let key_vector = builder.create_vector_direct(modification.key.as_bytes());

        let value_size = modification.value.len() as u16;

        let value_size = builder.create_vector_direct(&value_size.to_le_bytes());

        let value_vector = builder.create_vector_direct(modification.value.as_bytes());

        let mut modify_metadata = modify_metadata::MetadataModificationBufferBuilder::new(builder);
        modify_metadata.add_size_(modification.size() as u32);
        modify_metadata.add_modification_type(modification.r#type.value());
        modify_metadata.add_key_size(key_size as u8);
        modify_metadata.add_value_size(value_size);
        modify_metadata.add_key(key_vector);
        modify_metadata.add_value(value_vector);

        modification_buffer.push(modify_metadata.finish());
    }

    builder.create_vector(&modification_buffer).value()
}
