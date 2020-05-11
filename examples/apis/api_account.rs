use xpx_chain_apis::SiriusClient;
use xpx_chain_sdk::account::PublicAccount;
use xpx_chain_sdk::network::PUBLIC_TEST;

const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";

const PUBLIC_KEY_A: &str = "ECAA55A83E4EC110279A7C82010A52A3252B42905B0FDAA509F74A01FBBD38F2";

const PUBLIC_KEY_B: &str = "3B49BF0A08BB7528E54BB803BEEE0D935B2C800364917B6EFF331368A4232FD5";

#[tokio::main]
async fn main() {
    let sirius_client = SiriusClient::new(NODE_URL).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let public_account = PublicAccount::from_public_key(PUBLIC_KEY_A, PUBLIC_TEST).unwrap();

    let account_info = client.account_api().account_info(PUBLIC_KEY_A).await;
    match account_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{}", err),
    }

    let accounts_info = client
        .account_api()
        .accounts_info(vec![PUBLIC_KEY_A, PUBLIC_KEY_B])
        .await;
    match accounts_info {
        Ok(accounts) => accounts
            .iter()
            .for_each(|account_info| println!("{}", account_info)),
        Err(err) => eprintln!("{}", err),
    }

    let multisig = client
        .account_api()
        .account_multisig("VDPZJMY6D4LDBAHTAFDPZPLH5WQD4XTYHXQVFJLB")
        .await;
    match multisig {
        Ok(account_info) => println!("{}", account_info),
        Err(err) => eprintln!("{}", err),
    }

    let multisig = client
        .account_api()
        .account_multisig_graph("VDPZJMY6D4LDBAHTAFDPZPLH5WQD4XTYHXQVFJLB")
        .await;
    match multisig {
        Ok(account_info) => println!("{}", account_info),
        Err(err) => eprintln!("{}", err),
    }

    let accounts_transactions = client
        .account_api()
        .transactions(&public_account, None, None, Some("id"))
        .await;
    match accounts_transactions {
        Ok(accounts) => accounts
            .iter()
            .for_each(|account_txs| println!("{}", account_txs)),
        Err(err) => eprintln!("{}", err),
    }

    let accounts_names = client
        .account_api()
        .accounts_names(vec![PUBLIC_KEY_A, PUBLIC_KEY_B])
        .await;
    match accounts_names {
        Ok(account_names) => account_names
            .into_iter()
            .for_each(|account| println!("{}", account)),
        Err(err) => eprintln!("{}", err),
    }
}
