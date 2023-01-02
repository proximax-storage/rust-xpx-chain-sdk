use std::str::FromStr;

use xpx_chain_sdk::api::{ApiNode, TransactionQueryParams};
use xpx_chain_sdk::transaction::{TransactionGroupType, TransactionType};
use xpx_chain_sdk::TransactionHash;

#[tokio::main]
async fn main() -> Result<(), xpx_chain_sdk::api::error::Error> {
    let client = ApiNode::from_static("http://api-1.testnet2.xpxsirius.io:3000")
        .connect().await?;

    let transaction_api = client.transaction_api();

    let hash_one = TransactionHash::from_str(
        "33F586DB2235A57FAEA9A7F6A201F5AF525B368B812067019E49F4C8A350A397",
    )
        .unwrap();

    let hash_two = TransactionHash::from_str(
        "16134853ea5003d38bd2b0b48765eb7c5d58e48c7ab389dae040228a775c8f78",
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
    };

    Ok(())
}
