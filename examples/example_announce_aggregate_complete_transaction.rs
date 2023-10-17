/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::time::{Duration, Instant};

use xpx_chain_sdk::account::{Account, PublicAccount};
use xpx_chain_sdk::api::ApiNode;
use xpx_chain_sdk::message::PlainMessage;
use xpx_chain_sdk::mosaic::Mosaic;
use xpx_chain_sdk::transaction::{AggregateTransaction, Transaction, TransferTransaction};

#[tokio::main]
async fn main() -> Result<(), xpx_chain_sdk::api::error::Error> {
	let start = Instant::now();

	let client = ApiNode::builder()
		.uri("http://13.250.191.109:3000")
		// .timeout(Duration::from_secs(1))
		.connect_timeout(Duration::from_secs(5))
		.build()?
		.connect()
		.await?;

	let chain = client.chain_api().get_block_by_height(1).await?;

	// // let network_type = xpx_chain_sdk::network::PUBLIC_TEST;
	let network_type = chain.network_type;
	let generation_hash = chain.generation_hash;

	let private_key: &str = "7C57C8A6D8767B48F81E8E5D7ECBC76F60D55455A79B4008A20146185F7C8882";
	let signer_account = Account::from_hex_private_key(private_key, network_type).unwrap();

	println!("signer_account: {}", signer_account.address_str());

	let private_key: &str = "A3AB55F3C5EDE61C6D61BB6211C514F70048192A24DCE3D586912E1360378697";
	let account_a = Account::from_hex_private_key(private_key, network_type).unwrap();
	println!("signer_a: {}", account_a.address_str());

	let private_key: &str = "69C7B0C1982245A553C39FA1F3A934555C0715DD7415FC83DA277E7C9D1DD49C";
	let account_b = Account::from_hex_private_key(private_key, network_type).unwrap();
	println!("signer_b: {}", account_b.address_str());

	let public_key: &str = "8387fb7c444212c0f20d4952c1507756573c4c4f53874a1e0e4701983d895515";
	let sender_multi_s_account = PublicAccount::from_public_key(public_key, network_type).unwrap();
	println!("sender_account: {}", sender_multi_s_account.address_as_string());

	let recipient_a = Account::random(network_type).to_address();
	let recipient_b = Account::random(network_type).to_address();

	let mut transfer_a = TransferTransaction::builder(network_type)
		.recipient(recipient_a)
		.message(PlainMessage::create("Transfer A From ProximaX Rust SDK"))
		.mosaics(vec![Mosaic::xpx(11)])
		.build()?;
	transfer_a.set_aggregate(sender_multi_s_account);

	let mut transfer_b = TransferTransaction::builder(network_type)
		.recipient(recipient_b)
		.message(PlainMessage::create("Transfer B From ProximaX Rust SDK"))
		// .mosaics(vec![Mosaic::xpx(100)])
		.build()?;
	transfer_b.set_aggregate(sender_multi_s_account);

	let transaction = AggregateTransaction::builder_complete(network_type)
		.inner_transactions(vec![Box::new(transfer_a), Box::new(transfer_b)])
		// .max_fee(1000)
		.build()?;

	let sig_transaction = signer_account.sign_with_cosignatories(
		transaction,
		vec![account_a, account_b],
		generation_hash,
	)?;

	let response = client.transaction_api().announce(&sig_transaction).await?;
	println!("{response}");
	// println!("Singer: \t{}", account.public_key_to_hex());
	println!("Hash: \t{:X}", sig_transaction.hash);

	let duration = start.elapsed();

	println!("Time elapsed in expensive_function() is: {:?}", duration);
	Ok(())
}
