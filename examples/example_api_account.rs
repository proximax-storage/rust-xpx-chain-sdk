use xpx_chain_sdk::account::{Address, PublicAccount};
use xpx_chain_sdk::api::{OrderV2, SiriusClient, TransactionQueryParams};
use xpx_chain_sdk::transaction::TransactionType;

#[tokio::main]
async fn main() {
    let node_url = vec!["https://api-2.testnet2.xpxsirius.io:443"];

    let sirius_client = SiriusClient::new_from_urls(&node_url).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let network_type = client.network_type();

    let account_api = client.account_api();

    // let account_id: &str = "VCWN77662VNLFGQBQ6RIFTC7LFWO5BSA3U622CSK";
    let account_id: &str = "F6A9951DDB2B9EF2C7EB5132654562FC6D2308B42AEAAFD11097B232F429AF70";

    /// Create an Address from a given public key.
    let account_one = PublicAccount::from_public_key(
        "F6A9951DDB2B9EF2C7EB5132654562FC6D2308B42AEAAFD11097B232F429AF70",
        network_type,
    )
    .unwrap();

    /// Create an Address from a given public key.
    let account_two = Address::from_public_key(
        "F6A9951DDB2B9EF2C7EB5132654562FC6D2308B42AEAAFD11097B232F429AF70",
        network_type,
    )
    .unwrap();

    // Returns the account information.
    let account_info = account_api.account_info(account_id).await;
    match account_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("Exception when calling AccountApi->AccountInfo: {}\n", err),
    }

    // Returns the account information for an vector of accounts.
    let account_info = account_api.accounts_info(vec![account_id]).await;
    match account_info {
        Ok(resp) => println!("{:#?}", resp),
        Err(err) => eprintln!("Exception when calling AccountsApi->AccountInfo: {}\n", err),
    }

    // Returns the [multisig account] information.
    let account_info = account_api.account_multisig(account_id).await;
    match account_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!(
            "Exception when calling AccountApi->AccountMultisig: {}\n",
            err
        ),
    }

    // Returns the [multisig account] graph.
    let account_info = account_api.account_multisig_graph(account_id).await;
    match account_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!(
            "Exception when calling AccountApi->AccountMultisigGraph: {}\n",
            err
        ),
    }

    // Returns the [configurable properties] for a given account.
    let account_properties = account_api.account_properties(account_one.address).await;
    match account_properties {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!(
            "Exception when calling AccountApi->AccountProperties: {}\n",
            err
        ),
    }

    // Returns the [configurable properties] for a given vector of addresses.
    let accounts_properties = account_api
        .accounts_properties(vec![account_one.address, account_two])
        .await;
    match accounts_properties {
        Ok(resp) => println!("{:#?}", resp),
        Err(err) => eprintln!(
            "Exception when calling AccountsApi->AccountsProperties: {}\n",
            err
        ),
    }

    let txn_query_params = TransactionQueryParams {
        page_size: Some(15),
        page_number: Some(1),
        r#type: vec![
            TransactionType::AggregateComplete,
            TransactionType::Transfer,
        ],
        embedded: true,
        order: Some(OrderV2::DESC),
        sort_field: None,
        to_height: None,
        from_height: None,
        height: None,
        signer_public_key: None,
        recipient_address: None,
        address: None,
        public_key: None,
        first_level: false,
    };

    // Returns the [configurable properties] for a given vector of addresses.
    let accounts_properties = account_api
        .transactions(account_one, Some(txn_query_params))
        .await;
    match accounts_properties {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!(
            "Exception when calling AccountsApi->AccountsProperties: {}\n",
            err
        ),
    }
}
