use std::str::FromStr;

use xpx_chain_sdk::api::{SiriusClient, TransactionQueryParams};
use xpx_chain_sdk::transaction::{TransactionGroupType, TransactionType};
use xpx_chain_sdk::TransactionHash;

#[tokio::main]
async fn main() {
    let node_url = "http://demo-api-3.edlx.io:3000";
    // let node_url = "http://167.114.169.166:3000";
    // let node_url = "http://192.99.55.111:3000";

    let sirius_client = SiriusClient::new(node_url).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let transaction_api = client.transaction_api();

    let hash_one = TransactionHash::from_str(
        "fb2fa735982d8c43008bfb0bffc1c18715e84978165535dc4c4f4a6cf2d88d47",
    )
        .unwrap();

    let hash_two = TransactionHash::from_str(
        "47dec094a3379d9cfb1ed100c843da5f5578af7ad16bb2ef3d5d0ab0f6493c8d",
    )
        .unwrap();

    // // Returns a [Transaction] information given a transactionId or hash.
    // let account_info = transaction_api.get_any_transaction(hash_one).await;
    // match account_info {
    //     Ok(resp) => println!("{}", resp),
    //     Err(err) => eprintln!(
    //         "Exception when calling transaction_api->get_any_transaction: {}\n",
    //         err
    //     ),
    // }

    // // Returns a [Transaction] information given a transactionId or hash.
    // let account_info = transaction_api
    //     .get_transaction(hash_one, TransactionGroupType::Confirmed)
    //     .await;
    // match account_info {
    //     Ok(resp) => println!("{}", resp),
    //     Err(err) => eprintln!(
    //         "Exception when calling transaction_api->get_transaction: {}\n",
    //         err
    //     ),
    // }
    //
    // // Returns a [Transaction] information given a transactionId or hash.
    // let account_info = transaction_api
    //     .get_transactions(vec![hash_one, hash_two], TransactionGroupType::Confirmed)
    //     .await;
    // match account_info {
    //     Ok(resp) => println!("{:?}", resp),
    //     Err(err) => eprintln!(
    //         "Exception when calling transaction_api->get_transactions: {}\n",
    //         err
    //     ),
    // }
    //
    // let txn_query_params = TransactionQueryParams {
    //     embedded: true,
    //     r#type: vec![TransactionType::Transfer],
    //     signer_public_key: Some(
    //         "C2F93346E27CE6AD1A9F8F5E3066F8326593A406BDF357ACB041E2F9AB402EFE".to_string(),
    //     ),
    //     ..Default::default()
    // };
    //
    // // Returns a [Transaction] information given a transactionId or hash.
    // let account_info = transaction_api
    //     .get_transactions_by_group(TransactionGroupType::Confirmed, Some(txn_query_params))
    //     .await;
    // match account_info {
    //     Ok(resp) => println!("{}", resp),
    //     Err(err) => eprintln!(
    //         "Exception when calling transaction_api->get_transactions_by_group: {}\n",
    //         err
    //     ),
    // }
    //
    // // Returns a [Transaction] information given a transactionId or hash.
    // let account_info = transaction_api.get_transaction_status(hash_one).await;
    // match account_info {
    //     Ok(resp) => println!("{}", resp),
    //     Err(err) => eprintln!(
    //         "Exception when calling transaction_api->get_transaction_status: {}\n",
    //         err
    //     ),
    // }

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
