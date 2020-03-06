use hyper::Client;

use xpx_chain_sdk::sirius_client::SiriusClient;

const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";
const PUBLIC_KEY_A: &str = "93C3B9075649F59BD88573ADC55B8915B12390A47C76F0C45F362ED0800BE237";
const PUBLIC_KEY_B: &str = "3B49BF0A08BB7528E54BB803BEEE0D935B2C800364917B6EFF331368A4232FD5";

#[tokio::main]
async fn main() {
    let client = SiriusClient::new(NODE_URL, Client::new());

    let account_info = client.to_owned().account.get_account_info(PUBLIC_KEY_A).await;

    match account_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{:?}", err),
    }

    let accounts_info = client.account.get_accounts_info(
        vec![PUBLIC_KEY_A, PUBLIC_KEY_B]).await;

    match accounts_info {
        Ok(tx) => {
            for info in tx {
                println!("{}", info)
            }
        },
        Err(err) => eprintln!("{:?}", err),
    }
}
