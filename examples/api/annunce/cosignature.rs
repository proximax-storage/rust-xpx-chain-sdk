// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_apis::SiriusClient;
use xpx_chain_sdk::account::Account;
use xpx_chain_sdk::multisig::CosignatureTransaction;

const PRIVATE_KEY: &str = "EE5D1277A862A449173C55454740BEE1A29AB837A97507021340B6EA68909097";

#[tokio::main]
async fn main() {
    let node_url = vec!["http://bctestnet3.brimstone.xpxsirius.io:3000"];

    let sirius_client = SiriusClient::new(node_url).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };
    
    // let network_type = xpx_chain_sdk::network::PUBLIC_TEST;
    let network_type = client.network_type();
    
    let account = Account::from_private_key(PRIVATE_KEY, network_type).unwrap();
    
    println!("{}", account);
    
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
