#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::Client;

use xpx_chain_sdk::sirius_client::SiriusClient;

const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";

#[tokio::main]
async fn main() {
    let client = SiriusClient::new(NODE_URL, Client::new());

    let transaction_status = client.clone().transaction.get_transaction_status(
        "233E7A126483C4707FA57D366AF1D5A77F816607F06A74A460A3F7B84BB63648").await;
    match transaction_status {
        Ok(status) => println!("{}", status),
        Err(err) => eprintln!("{}", err),
    }

    let transactions_ids = vec![
        "130171141CAE9D9ED6F62FD47CC316631986BBACD6B3D63930A9C46ED1ED764F",
        "5EC5C0E766B3DF81FBAD0E4FD794828002763905FEDC47208520E90FBED783B4"
    ];

    let transactions_statuses = client.clone().transaction.get_transactions_statuses(
        transactions_ids.clone()).await;
    match transactions_statuses {
        Ok(statuses) => {
            statuses.iter().for_each(|status|{
                println!("{}", status)
            })
        }
        Err(err) => eprintln!("{}", err),
    }

    let transaction = client.clone().transaction.get_transaction(
        "81F4D14A79A9902D1E74E6CEAE839F8FCC3B43A60667DFBE627805A486D4D99C").await;
    match transaction {
        Ok(tx) => println!("{}", tx),
        Err(err) => eprintln!("{}", err),
    }

    let transactions = client.clone().transaction.get_transactions(transactions_ids).await;
    match transactions {
        Ok(txs) => {
            txs.iter().for_each(|tx_info|{
                println!("{}", tx_info)
            })
        }
        Err(err) => eprintln!("{}", err),
    }
}
