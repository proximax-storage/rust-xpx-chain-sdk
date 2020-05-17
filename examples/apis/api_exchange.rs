#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_apis::SiriusClient;
use xpx_chain_sdk::account::PublicAccount;

const NODE_URL: &str = "http://bcstage1.xpxsirius.io:3000";

const PUBLIC_KEY: &str = "0A233A17473F77A6DC0FA2B707D70B370B51E7E3C47A9C6D8F74341453121726";

#[tokio::main]
async fn main() {
    let sirius_client = SiriusClient::new(NODE_URL).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let network_type = client.network_type();

    let public_account = PublicAccount::from_public_key(
        PUBLIC_KEY,
        network_type,
    ).unwrap();

    let node_info = client
        .exchange_api()
        .get_account_exchange_info(public_account)
        .await;

    match node_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{}", err),
    }
}
