#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_apis::SiriusClient;

#[tokio::main]
async fn main() {
    let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];

    let sirius_client = SiriusClient::new(node_url).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let transaction_status = client
        .transaction_api()
        .get_transaction("BD52CCEE7A99BB9E46EA3011A5EA03E892785972D28FBFF72C560FC38B64C073")
        .await;

    match transaction_status {
        Ok(status) => println!("{}", status),
        Err(err) => eprintln!("{}", err),
    }
    //
    // let transactions_ids = vec![
    //     "E55037A661EE69C5F45CD3F40744F0923DD5B827F50BAF9AAF61DF649DC7B1D9",
    //     "5EC5C0E766B3DF81FBAD0E4FD794828002763905FEDC47208520E90FBED783B4",
    // ];
    //
    // let transactions_statuses = client
    //     .transaction_api()
    //     .get_transactions_statuses(transactions_ids.clone())
    //     .await;
    //
    // match transactions_statuses {
    //     Ok(statuses) => statuses.iter().for_each(|status|
    //         println!("{}", status)
    //     ),
    //     Err(err) => eprintln!("{}", err),
    // }
    //
    // let transaction = client
    //     .transaction_api()
    //     .get_transaction("D02D59450AE5A41F50B0930671FD2BC1233CBA44049DDB6F86DB7409B5551A73")
    //     .await;
    //
    // match transaction {
    //     Ok(tx) => println!("{}", tx),
    //     Err(err) => eprintln!("{}", err),
    // }
    //
    // let transactions = client
    //     .transaction_api()
    //     .get_transactions(transactions_ids)
    //     .await;
    //
    // match transactions {
    //     Ok(txs) => txs.into_iter().for_each(|tx_info|
    //         println!("{}", tx_info)
    //     ),
    //     Err(err) => eprintln!("{}", err),
    // }
}
