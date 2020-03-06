#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::Client;

use xpx_chain_sdk::sirius_client::SiriusClient;

#[tokio::main]
async fn main() {
    let node = "http://bctestnet3.brimstone.xpxsirius.io:3000";

    let client = SiriusClient::new(node, Client::new());

    let transaction_status = client.clone().transaction.get_transaction_status(
        "A5215CA0D024B50F59B6A19354638F2A366B44B08A95DA59A926250400BBE86E").await;
    match transaction_status {
        Ok(status) => println!("{}", status),
        Err(err) => eprintln!("{:?}", err),
    }

    let transactions_ids = vec![
        "130171141CAE9D9ED6F62FD47CC316631986BBACD6B3D63930A9C46ED1ED764F",
        "5EC5C0E766B3DF81FBAD0E4FD794828002763905FEDC47208520E90FBED783B4"
    ];

    let transactions_statuses = client.clone().transaction.get_transactions_statuses(
        transactions_ids.clone()).await;
    match transactions_statuses {
        Ok(statuses) => {
            for status in statuses {
                println!("{}", status)
            }
        }
        Err(err) => eprintln!("{:?}", err),
    }

    let transaction = client.clone().transaction.get_transaction(
        "130171141CAE9D9ED6F62FD47CC316631986BBACD6B3D63930A9C46ED1ED764F").await;
    match transaction {
        Ok(tx) => println!("{}", tx),
        Err(err) => eprintln!("{:?}", err),
    }

    let transactions = client.clone().transaction.get_transactions(transactions_ids).await;
    match transactions {
        Ok(tx) => {
            for i in tx {
                println!("{}", i)
            }
        },
        Err(err) => eprintln!("{:?}", err),
    }
}
