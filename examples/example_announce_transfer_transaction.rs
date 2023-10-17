/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::time::{Duration, Instant};

use xpx_chain_sdk::account::{Account, Address};
use xpx_chain_sdk::api::ApiNode;
use xpx_chain_sdk::mosaic::{Mosaic, MosaicId};
use xpx_chain_sdk::transaction::TransferTransaction;

#[tokio::main]
async fn main() -> Result<(), xpx_chain_sdk::api::error::Error> {
	let start = Instant::now();

	let client = ApiNode::builder()
		.uri("http://18.141.176.191:3000")
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

	let account = Account::from_hex_private_key(private_key, network_type).unwrap();
	let recipient = Address::from_public_key(
		"4b8a8cd9071f870f07e0714dc902d07604593f491c33af5ee97fb784c19670b9",
		network_type,
	)
	.unwrap();

	let asset_id = MosaicId::try_from_hex("6291690CAB3C593D")?;

	let transfer_transaction = TransferTransaction::builder(network_type)
		.recipient(recipient)
		// .max_fee(1000)
		.mosaics(vec![Mosaic::create(asset_id, 1000000000).unwrap()])
		.build()?;

	let sig_transaction = account.sign_transaction(transfer_transaction, generation_hash)?;

	let response = client.transaction_api().announce(&sig_transaction).await?;
	println!("{response}");
	println!("Singer: \t{}", account.public_key_to_hex());
	println!("Hash: \t{:X}", sig_transaction.hash);

	let duration = start.elapsed();

	println!("Time elapsed in expensive_function() is: {:?}", duration);
	Ok(())
}
