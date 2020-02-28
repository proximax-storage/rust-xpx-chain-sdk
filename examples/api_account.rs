#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::Client;

use xpx_chain_sdk::apis::sirius_client::SiriusClient;

#[tokio::main]
async fn main() {
    let node = "http://bctestnetswap.xpxsirius.io:3000";

    let client = SiriusClient::new(node, Client::new());

    let account_info = client.account.get_account_info(
        "93C3B9075649F59BD88573ADC55B8915B12390A47C76F0C45F362ED0800BE237").await;

    match account_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => panic!("{:?}", err),
    }
}
