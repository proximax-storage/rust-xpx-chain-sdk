use std::str::FromStr;

use xpx_chain_sdk::api::{SiriusClient, TransactionQueryParams};
use xpx_chain_sdk::transaction::{TransactionGroupType, TransactionType};
use xpx_chain_sdk::TransactionHash;

#[tokio::main]
async fn main() {
    let node_url = vec!["https://api-2.testnet2.xpxsirius.io:443"];

    let sirius_client = SiriusClient::new_from_urls(&node_url).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let transaction_api = client.transaction_api();

    let hash_one = TransactionHash::from_str(
        "2E1BC6D0B59564649B0DEC3211398A467D4191847501863CC7342F2190CEF871",
    )
    .unwrap();

    let hash_two = TransactionHash::from_str(
        "BBBAB6316868DC400F27B293FDE6AEFBC30295F38978CD472787C4C5C56F378B",
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
            "C2F93346E27CE6AD1A9F8F5E3066F8326593A406BDF357ACB041E2F9AB402EFE".to_string(),
        ),
        ..Default::default()
    };

    // Returns a [Transaction] information given a transactionId or hash.
    let account_info = transaction_api
        .get_transactions_by_group(TransactionGroupType::Confirmed, Some(txn_query_params))
        .await;
    match account_info {
        Ok(resp) => println!("{}", resp),
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
