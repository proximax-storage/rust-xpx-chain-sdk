#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::Client;

use xpx_chain_sdk::apis::sirius_client::SiriusClient;

#[tokio::main]
async fn main() {
    let node = "http://bctestnetswap.xpxsirius.io:3000";

    let client = SiriusClient::new(node, Client::new());
    let transaction_status = client.clone().transaction.get_transaction_status(
        "5BD1AE427B10E50902DE8037B7A551104727732B2C97A08A3DD19342714337A5"
    ).await;

    match transaction_status {
        Ok(status) => println!("{}", status),
        Err(err) => eprintln!("RESP_ERROR: {:?}", err),
    }

    let transactions_ids = vec![
        "5BD1AE427B10E50902DE8037B7A551104727732B2C97A08A3DD19342714337A5",
        "c088f96252ab76ac73b4367de2c8c6d7dfaf33ea3e7120d867888632862eb514"
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
        "11AE3717294B61F8A747ABBA793A63BEF65F0E218A0E5B9B7D33D571FEC1CA31"
    ).await;

    match transaction {
        Ok(tx) => println!("{}", tx),
        Err(err) => eprintln!("{:?}", err),
    }

    let transactions = client.clone().transaction.get_transactions(transactions_ids ).await;

    match transactions {
        Ok(tx) => {
            for i in tx {
                println!("{}", i)

            }
        },
        Err(err) => eprintln!("RESP_ERROR: {:?}", err),
    }
}
