/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::str::FromStr;

use xpx_chain_sdk::api::{ApiNode, TransactionQueryParams};
use xpx_chain_sdk::transaction::{TransactionGroupType, TransactionType};
use xpx_chain_sdk::TransactionHash;

#[tokio::main]
async fn main() -> Result<(), xpx_chain_sdk::api::error::Error> {
	let client = ApiNode::from_static("http://bctestnet3.brimstone.xpxsirius.io:3000").connect_lazy();
	// .await?;

	let transaction_api = client.transaction_api();
	let hash_one = TransactionHash::from_str(
		"C1BCCC310D1EF17553160A14213949E35DA229D3D9805EED1D00C4E5DD93AE86",
	)
	.unwrap();

	let hash_two = TransactionHash::from_str(
		"4E86FB6372C3783E6EA28F48DBB600B727C0BC8C9BD81CE6CC9DCB9520230990",
	)
	.unwrap();

	// Returns a [Transaction] information given a transactionId or hash.
	let account_info = transaction_api.get_any_transaction(hash_one).await;
	match account_info {
		Ok(resp) => println!("{}", resp),
		Err(err) => {
			eprintln!("Exception when calling transaction_api->get_any_transaction: {}\n", err)
		},
	}

	// Returns a [Transaction] information given a transactionId or hash.
	let account_info =
		transaction_api.get_transaction(hash_one, TransactionGroupType::Confirmed).await;
	match account_info {
		Ok(resp) => println!("{}", resp),
		Err(err) => eprintln!("Exception when calling transaction_api->get_transaction: {}\n", err),
	}

	// Returns a [Transaction] information given a transactionId or hash.
	let account_info = transaction_api
		.get_transactions(vec![hash_one, hash_two], TransactionGroupType::Confirmed)
		.await;
	match account_info {
		Ok(resp) => println!("{:#?}", resp),
		Err(err) => {
			eprintln!("Exception when calling transaction_api->get_transactions: {}\n", err)
		},
	}

	let txn_query_params = TransactionQueryParams::create()
		.public_key("c2f93346e27ce6ad1a9f8f5e3066f8326593a406bdf357acb041e2f9ab402efe".to_string())
		.r#type(vec![TransactionType::Transfer])
		.embedded(true)
		.build()
		.unwrap();

	// Returns a [Transaction] information given a transactionId or hash.
	let account_info = transaction_api
		.get_transactions_by_group(TransactionGroupType::Confirmed, Some(txn_query_params))
		.await;
	match account_info {
		Ok(resp) => resp.transactions.iter().for_each(|item| println!("{:#?}", item)),
		Err(err) => eprintln!(
			"Exception when calling transaction_api->get_transactions_by_group: {}\n",
			err
		),
	}

	// Returns a [Transaction] information given a transactionId or hash.
	let account_info = transaction_api.get_transaction_status(hash_one).await;
	match account_info {
		Ok(resp) => println!("{}", resp),
		Err(err) => {
			eprintln!("Exception when calling transaction_api->get_transaction_status: {}\n", err)
		},
	}

	// Returns a [Transaction] information given a transactionId or hash.
	let account_info = transaction_api.get_transactions_statuses(vec![hash_one]).await;
	match account_info {
		Ok(resp) => println!("{:#?}", resp),
		Err(err) => eprintln!(
			"Exception when calling transaction_api->get_transactions_statuses: {}\n",
			err
		),
	};

	Ok(())
}
