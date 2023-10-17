/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::time::Duration;

use xpx_chain_sdk::account::{Account, Address, PublicAccount};
use xpx_chain_sdk::api::ApiNode;
use xpx_chain_sdk::api::{OrderV2, TransactionQueryParams};
use xpx_chain_sdk::transaction::TransactionType;

#[tokio::main]
async fn main() -> Result<(), xpx_chain_sdk::api::error::Error> {
	let client = ApiNode::builder()
		.uri("http://bctestnet3.brimstone.xpxsirius.io:3000")
		// .timeout(Duration::from_secs(1))
		.connect_timeout(Duration::from_secs(5))
		.max_num_retry(2)
		.concurrency_limit(100)
		.build()?
		.connect()
		.await?;

	let network_type = client.chain_api().get_block_by_height(1).await?.network_type;
	// let network_type = NetworkType::PublicTest;

	// let account_api = client.account_api();

	// let account_id_one: &str = "VCTVW23D2MN5VE4AQ4TZIDZENGNOZXPRPR3HTEHT";
	let account_id_one: &str = "7C57C8A6D8767B48F81E8E5D7ECBC76F60D55455A79B4008A20146185F7C8882";

	// let account_id_two: &str = "VCWN77662VNLFGQBQ6RIFTC7LFWO5BSA3U622CSK";
	let account_id_two: &str = "b0fd7cea2a03d7287551771d3395249546a2c32a5df6ecb54f";

	// Create an Address from a given public key.
	let account_one = Account::random(network_type);
	println!("public_key_to_hex: {}", account_one.public_key_to_hex());
	println!("private_key_to_hex: {}", account_one.private_key_to_hex());

	// Create an Address from a given public key.
	let account_two = Address::from_encoded(account_id_two).unwrap();

	println!("{account_two}");

	let account_api = client.account_api();

	for _i in 0..100 {
		let account_api = account_api.clone();

		tokio::spawn(async move {
			let req = account_api.account_info(account_id_one).await.unwrap();
			println!("{:?}", req);
		});
	}

	// Returns the account information for an vector of accounts.
	let account_info = account_api.accounts_info(vec![account_id_one, account_id_two]).await;
	match account_info {
		Ok(resp) => println!("{:#?}", resp),
		Err(err) => eprintln!("Exception when calling AccountsApi->AccountInfo: {}\n", err),
	}

	// Returns the [multisig account] information.
	let account_info = account_api.account_multisig(account_id_one).await;
	match account_info {
		Ok(resp) => println!("{}", resp),
		Err(err) => eprintln!("Exception when calling AccountApi->AccountMultisig: {}\n", err),
	}

	// Returns the [multisig account] graph.
	let account_info = account_api.account_multisig_graph(account_id_one).await;
	match account_info {
		Ok(resp) => println!("{}", resp),
		Err(err) => eprintln!("Exception when calling AccountApi->AccountMultisigGraph: {}\n", err),
	}

	// Returns the [configurable properties] for a given account.
	let account_properties = account_api.account_properties(account_one.address).await;
	match account_properties {
		Ok(resp) => println!("{}", resp),
		Err(err) => eprintln!("Exception when calling AccountApi->AccountProperties: {}\n", err),
	}

	// Returns the [configurable properties] for a given vector of addresses.
	let accounts_properties =
		account_api.accounts_properties(vec![account_one.address, account_two]).await;
	match accounts_properties {
		Ok(resp) => println!("{:#?}", resp),
		Err(err) => eprintln!("Exception when calling AccountsApi->AccountsProperties: {}\n", err),
	}

	let txn_query_params = TransactionQueryParams::create()
		.page_size(10_u16)
		.page_number(1_u16)
		.r#type(vec![TransactionType::AggregateComplete, TransactionType::Transfer])
		.embedded(true)
		.order(OrderV2::DESC)
		.first_level(false)
		.build()
		.unwrap();

	// Returns the [configurable properties] for a given vector of addresses.
	let accounts_properties = account_api.transactions(account_one, Some(txn_query_params)).await;
	match accounts_properties {
		Ok(resp) => println!("{}", resp),
		Err(err) => eprintln!("Exception when calling AccountsApi->AccountsProperties: {}\n", err),
	}

	Ok(())
}
