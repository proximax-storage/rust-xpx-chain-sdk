// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_apis::SiriusClient;
use xpx_chain_sdk::account::PublicAccount;

const PUBLIC_KEY_A: &str = "c8f52a6ed98c5bcd52e090da0d1950d58b13d239e4cecc05f5d4acd706f5da75";
const PUBLIC_KEY_B: &str = "3B49BF0A08BB7528E54BB803BEEE0D935B2C800364917B6EFF331368A4232FD5";

#[tokio::main]
async fn main() {
    let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];

    let sirius_client = SiriusClient::new(node_url).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    // let network_type = xpx_chain_sdk::network::PUBLIC_TEST;
    let network_type = client.network_type();
    
    let public_account = PublicAccount::from_public_key(PUBLIC_KEY_A, network_type).unwrap();
    
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
        .account_multisig("VDPZJM-Y6D4LD-BAHTAF-DPZPLH-5WQD4X-TYHXQV-FJLB")
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
    
    let tx_partial = client
        .account_api()
        .partial_transactions(&public_account, None, None, None)
        .await;
    
    match tx_partial {
        Ok(txs) => txs.into_iter().for_each(|tx| println!("{}", tx)),
        Err(err) => eprintln!("{}", err),
    }
}
