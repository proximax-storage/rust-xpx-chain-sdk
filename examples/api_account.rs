use hyper::Client;

use xpx_chain_sdk::account::PublicAccount;
use xpx_chain_sdk::network::PUBLIC_TEST;
use xpx_chain_sdk::sirius_client::SiriusClient;
use std::time::Duration;

const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";
const PUBLIC_KEY_A: &str = "93C3B9075649F59BD88573ADC55B8915B12390A47C76F0C45F362ED0800BE237";
const PUBLIC_KEY_B: &str = "3B49BF0A08BB7528E54BB803BEEE0D935B2C800364917B6EFF331368A4232FD5";

#[tokio::main]
async fn main() {
    let sirius_client = SiriusClient::new(NODE_URL, Client::new()).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let public_account = PublicAccount::from_public_key(PUBLIC_KEY_B, PUBLIC_TEST).unwrap();

    let account_info = client.to_owned().account.account_info(PUBLIC_KEY_A).await;
    match account_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{}", err),
    }

    let accounts_info = client.clone().account.accounts_info(vec![PUBLIC_KEY_A, PUBLIC_KEY_B]).await;
    match accounts_info {
        Ok(accounts) => {
            accounts.iter().for_each(|account_info| {
                println!("{}", account_info)
            })
        }
        Err(err) => eprintln!("{}", err),
    }

    let accounts_transactions = client.account.incoming_transactions(public_account, None, None, Some("id")).await;
    match accounts_transactions {
        Ok(accounts) => {
            accounts.iter().for_each(|account_txs| {
                println!("{}", account_txs)
            })
        }
        Err(err) => eprintln!("{}", err),
    }
}
