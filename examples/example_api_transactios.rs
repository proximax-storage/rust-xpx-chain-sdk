use std::str::FromStr;

use xpx_chain_sdk::api::{Client, Endpoint, TransactionQueryParams};
use xpx_chain_sdk::transaction::{TransactionGroupType, TransactionType};
use xpx_chain_sdk::TransactionHash;

#[tokio::main]
async fn main() {
    let ep1 = Endpoint::from_static("http://api-1.testnet2.xpxsirius.io:3000");
    let ep2 = Endpoint::from_static("http://api-2.testnet2.xpxsirius.io:3000");

    let endpoints = [ep1, ep2];

    let client = Client::balance_list(endpoints.into_iter());
    let transaction_api = client.transaction_api();

    let hash_one = TransactionHash::from_str(
        "B27577726AD8F7CB848891F9AC585FE89252DF40327EDF29DE6C3CA9295AE074",
    )
        .unwrap();

    let hash_two = TransactionHash::from_str(
        "47dec094a3379d9cfb1ed100c843da5f5578af7ad16bb2ef3d5d0ab0f6493c8d",
    )
        .unwrap();

    // Returns a [Transaction] information given a transactionId or hash.
    let account_info = transaction_api.get_any_transaction(hash_one).await;
    match account_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!(
            "Exception when calling transaction_api->get_any_transaction: {}\n",
            err
        ),
    }

    // Returns a [Transaction] information given a transactionId or hash.
    let account_info = transaction_api
        .get_transaction(hash_one, TransactionGroupType::Confirmed)
        .await;
    match account_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!(
            "Exception when calling transaction_api->get_transaction: {}\n",
            err
        ),
    }

    // Returns a [Transaction] information given a transactionId or hash.
    let account_info = transaction_api
        .get_transactions(vec![hash_one, hash_two], TransactionGroupType::Confirmed)
        .await;
    match account_info {
        Ok(resp) => println!("{:?}", resp),
        Err(err) => eprintln!(
            "Exception when calling transaction_api->get_transactions: {}\n",
            err
        ),
    }

    let txn_query_params = TransactionQueryParams {
        embedded: true,
        r#type: vec![TransactionType::Transfer],
        signer_public_key: Some(
            "5e1d44d8a09b2146cc64f369d4b030ce8df4f6d4bee499aeb62f6d154cf95fa4".to_string(),
        ),
        ..Default::default()
    };

    // Returns a [Transaction] information given a transactionId or hash.
    let account_info = transaction_api
        .get_transactions_by_group(TransactionGroupType::Confirmed, Some(txn_query_params))
        .await;
    match account_info {
        Ok(resp) => resp
            .transactions
            .iter()
            .for_each(|item| println!("{:?}", item.get_common_transaction().transaction_info)),
        Err(err) => eprintln!(
            "Exception when calling transaction_api->get_transactions_by_group: {}\n",
            err
        ),
    }

    // Returns a [Transaction] information given a transactionId or hash.
    let account_info = transaction_api.get_transaction_status(hash_one).await;
    match account_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!(
            "Exception when calling transaction_api->get_transaction_status: {}\n",
            err
        ),
    }

    // Returns a [Transaction] information given a transactionId or hash.
    let account_info = transaction_api
        .get_transactions_statuses(vec![hash_one, hash_two])
        .await;
    match account_info {
        Ok(resp) => println!("{:#?}", resp),
        Err(err) => eprintln!(
            "Exception when calling transaction_api->get_transactions_statuses: {}\n",
            err
        ),
    }
}
