use std::thread::sleep;
use std::time::Duration;



use xpx_chain_sdk::account::{Account, PublicAccount};
use xpx_chain_sdk::multisig::CosignatureTransaction;
use xpx_chain_sdk::network::PUBLIC_TEST;
use xpx_chain_sdk::sirius_client::SiriusClient;
use xpx_chain_sdk::transaction::AggregateTransaction;

const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";
const PRIVATE_KEY: &str = "28CF06338133DEE64FC49BCB19C8936916DBE8DC461CE489BF9588BE3B9670B5";

#[tokio::main]
async fn main() {
    let sirius_client = SiriusClient::new(NODE_URL).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let account = Account::from_private_key(PRIVATE_KEY, PUBLIC_TEST).unwrap();

    let partial = client
        .account_api()
        .partial_transactions(&account.public_account, None, None, None)
        .await;
    match partial {
        Ok(tx) => {
            for tx_partial in tx.into_iter() {
                println!("Hash: {}", tx_partial.transaction_hash());

                let cosigner_tx = CosignatureTransaction::new(tx_partial);
                if let Err(err) = cosigner_tx {
                    panic!(err)
                }

                let signed = account.sign_cosignature_transaction(cosigner_tx.unwrap());
                let signed_transaction = match signed {
                    Ok(resp) => resp,
                    Err(err) => panic!("{}", err),
                };

                let announce = client
                    .transaction_api()
                    .announce_aggregate_bonded_cosignature(&signed_transaction)
                    .await;
                match announce {
                    Ok(resp) => println!("{} \n", resp),
                    Err(err) => panic!("{}", err),
                };
            }
        }
        Err(err) => eprintln!("{}", err),
    }
}
