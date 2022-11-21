use http::{Method, Request};
use std::thread::sleep;
use std::time::Duration;
use tower::ServiceExt;
use xpx_chain_sdk::account::{Address, PublicAccount};
use xpx_chain_sdk::api::body::BoxBody;
use xpx_chain_sdk::api::routes::account_routes_api::AccountRoutes;
use xpx_chain_sdk::api::{Client, Endpoint, OrderV2, TransactionQueryParams};
use xpx_chain_sdk::network::NetworkType;
use xpx_chain_sdk::transaction::TransactionType;

#[tokio::main]
async fn main() {
    let endpoints = [
        "http://142.44.246.241:3000",
        "http://167.114.169.166:3000",
        "http://192.99.55.111:3000",
    ]
    .into_iter()
    .map(|a| {
        Client::from_shared(a).unwrap()
        // .rate_limit(10, Duration::from_millis(500))
        // .concurrency_limit(6)
        // .timeout(Duration::from_secs(15))
    });

    let channel = Client::balance_list(endpoints.into_iter());

    // let network_type = client.network_info().await.unwrap().network_type;
    let network_type = NetworkType::PublicTest;

    // let account_api = client.account_api();

    // let account_id_one: &str = "VCTVW23D2MN5VE4AQ4TZIDZENGNOZXPRPR3HTEHT";
    let account_id_one: &str = "50906aea8e57fe26fa6c0ca2e677f5e2572d81135aca50d0c339f7da88e915bc";

    // let account_id_two: &str = "VCWN77662VNLFGQBQ6RIFTC7LFWO5BSA3U622CSK";
    let account_id_two: &str = "F6A9951DDB2B9EF2C7EB5132654562FC6D2308B42AEAAFD11097B232F429AF70";

    // Create an Address from a given public key.
    let account_one = PublicAccount::from_public_key(account_id_one, network_type).unwrap();

    // Create an Address from a given public key.
    let account_two = Address::from_public_key(account_id_two, network_type).unwrap();

    let account_api = channel.account_api();

    for i in 0..3 {
        let account_api = account_api.clone();

        tokio::spawn(async move {
            let req = account_api.account_info(account_id_one).await;

            println!("ALGODON {:?}", req);
        });
    }

    // Returns the account information for an vector of accounts.
    let account_info = account_api
        .accounts_info(vec![account_id_one, account_id_two])
        .await;
    match account_info {
        Ok(resp) => println!("{:#?}", resp),
        Err(err) => eprintln!("Exception when calling AccountsApi->AccountInfo: {}\n", err),
    }

    // Returns the [multisig account] information.
    let account_info = account_api.account_multisig(account_id_one).await;
    match account_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!(
            "Exception when calling AccountApi->AccountMultisig: {}\n",
            err
        ),
    }

    // Returns the [multisig account] graph.
    let account_info = account_api.account_multisig_graph(account_id_one).await;
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
        first_level: false,
        ..Default::default()
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
    sleep(Duration::from_secs(100));
}
