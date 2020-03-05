use hyper::Client;

use xpx_chain_sdk::apis::sirius_client::SiriusClient;

const NODE_URL: &str = "http://bctestnetswap.xpxsirius.io:3000";
const PUBLIC_KEY: &str = "93C3B9075649F59BD88573ADC55B8915B12390A47C76F0C45F362ED0800BE237";

#[tokio::main]
async fn main() {
    let client = SiriusClient::new(NODE_URL, Client::new());

    let account_info = client.account.get_account_info(PUBLIC_KEY).await;

    match account_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{:?}", err),
    }
}
