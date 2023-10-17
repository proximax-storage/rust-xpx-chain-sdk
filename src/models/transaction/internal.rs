/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use anyhow::Result;
use fb::FlatBufferBuilder;

use crate::{
	account::Account,
	errors_const::ERR_EMPTY_TRANSACTION_SIGNER,
	helpers::{hex_decode, hex_encode, u32_to_array_u8, TransactionHash},
	metadata::MetadataModification,
	models::consts::{
		SIGNATURE_SIZE, SIGNER_SIZE, SIZE_SIZE, TRANSACTION_HEADER_SIZE, TYPE_SIZE, VERSION_SIZE,
	},
	mosaic::MosaicProperty,
	multisig::CosignatoryModification,
	AsUint64,
};

use super::{
	buffers::{
		CosignatoryModificationBuffer, CosignatoryModificationBufferBuilder,
		MetadataModificationBuffer, MetadataModificationBufferBuilder, MosaicPropertyBuilder,
	},
	AggregateTransaction, SignedTransaction, Transaction, TransactionVersion,
};

pub(crate) fn extract_version(version: u32) -> TransactionVersion {
	TransactionVersion((version & 0xFFFFFF) as u8)
}

pub(crate) fn sign_transaction_with_cosignatures(
	tx: AggregateTransaction,
	account: Account,
	cosignatories: Vec<Account>,
	generation_hash: TransactionHash,
) -> Result<SignedTransaction> {
	let entity_type = tx.get_transaction_type();
	let sig_txn = tx.sign_with(account, generation_hash)?;

	let mut payload = sig_txn.payload;
	cosignatories.iter().for_each(|item| {
		let key_pair: crypto::Keypair =
			crypto::Keypair::from_private_key(item.to_owned().key_pair.secret);
		let hash_bytes = sig_txn.hash.as_bytes();
		let signature = key_pair.sign(hash_bytes);
		payload.push_str(&format!(
			"{}{}",
			item.public_key_to_hex(),
			hex_encode(&signature.to_bytes()[..])
		));
	});

	let mut payload_bytes = hex_decode(&payload);

	let new_size = (payload_bytes.len() as i32).to_le_bytes();

	payload_bytes.splice(..4, new_size);

	Ok(SignedTransaction { entity_type, hash: sig_txn.hash, payload: hex_encode(&payload_bytes) })
}

pub(crate) fn mosaic_property_array_to_buffer(
	builder: &mut FlatBufferBuilder,
	properties: Vec<MosaicProperty>,
) -> fb::UOffsetT {
	let mut p_buffer: Vec<fb::UOffsetT> = Vec::with_capacity(properties.len());

	for p in properties {
		let value_v = builder.create_vector(&p.value.to_dto());

		let mut mosaic_property = MosaicPropertyBuilder::new(builder);
		mosaic_property.add_mosaic_property_id(p.id.value());
		mosaic_property.add_value(value_v);

		p_buffer.push(mosaic_property.finish().value());
	}

	builder.create_vector(&p_buffer).value()
}

pub(crate) fn to_aggregate_transaction_bytes(tx: &Box<dyn Transaction>) -> Result<Vec<u8>> {
	ensure!(tx.get_common_transaction().signer.is_some(), ERR_EMPTY_TRANSACTION_SIGNER);

	let signer_bytes = tx.get_common_transaction().signer.unwrap();

	let tx_bytes = tx.to_serializer();

	let mut r_b: Vec<u8> = Vec::with_capacity(tx_bytes.len());
	r_b.extend_from_slice(&[0u8; 4]);
	r_b.extend_from_slice(signer_bytes.to_builder());
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
	let mut cosignatory_buffer: Vec<fb::WIPOffset<CosignatoryModificationBuffer<'a>>> =
		Vec::with_capacity(modifications.len());

	for modification in modifications {
		let public_key = &modification.public_account;

		let public_key_vector = builder.create_vector(public_key.to_builder());

		let mut modify_multisig = CosignatoryModificationBufferBuilder::new(builder);
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
	let mut modification_buffer: Vec<fb::WIPOffset<MetadataModificationBuffer<'a>>> =
		Vec::with_capacity(modifications.len());

	for modification in modifications {
		let key_size = modification.key.len();
		if key_size == 0 {
			return 0;
		}

		let key_vector = builder.create_vector(modification.key.as_bytes());

		let value_size = modification.value.len() as u16;

		let value_size = builder.create_vector(&value_size.to_le_bytes());

		let value_vector = builder.create_vector(modification.value.as_bytes());

		let mut modify_metadata = MetadataModificationBufferBuilder::new(builder);
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
