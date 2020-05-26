// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.
#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_apis::SiriusClient;
use xpx_chain_sdk::account::Address;
use xpx_chain_sdk::namespace::NamespaceId;

#[tokio::main]
async fn main() {
    let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];

    let sirius_client = SiriusClient::new(node_url).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let network_type = client.network_type();

    let address_one = Address::from_public_key(
        "C952A761C0D51940AE77EC44DE93662133B5A2E93F5DCADAB7F972FA91F5DFCD",
        network_type,
    ).unwrap();

    let address_two = Address::from_raw(
        "VCVF646H3M3C5CNIVWFZ734NC2WQXWYUKBGIZAB5"
    ).unwrap();

    let namespace_one = NamespaceId::from_name("rustnamespace").unwrap();

    let namespace_two = NamespaceId::from("BFFB42A19116BDF6");

    let namespace_info = client
        .namespace_api()
        .get_namespace_info(namespace_one)
        .await;

    match namespace_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{}", err),
    }

    let from_account = client
        .namespace_api()
        .get_namespaces_from_account(address_one.clone(), None, None)
        .await;

    match from_account {
        Ok(namespaces) => namespaces
            .iter()
            .for_each(|namespace_info| println!("{}", namespace_info)),
        Err(err) => eprintln!("{}", err),
    }

    let namespaces_names = client
        .namespace_api()
        .get_namespaces_names(vec![namespace_one, namespace_two])
        .await;

    match namespaces_names {
        Ok(namespaces) => namespaces
            .iter()
            .for_each(|namespace_name| println!("{}", namespace_name)),
        Err(err) => eprintln!("{}", err),
    }

    let namespaces_accounts = client
        .namespace_api()
        .get_namespaces_from_accounts(vec![&address_one.address, &address_two.address], None, None)
        .await;

    match namespaces_accounts {
        Ok(namespaces) => namespaces
            .iter()
            .for_each(|namespace_name| println!("{}", namespace_name)),
        Err(err) => eprintln!("{}", err),
    }
}
