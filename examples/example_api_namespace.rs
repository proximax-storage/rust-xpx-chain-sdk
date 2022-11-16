/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use xpx_chain_sdk::account::{Address, PublicAccount};
use xpx_chain_sdk::api::{Client, Endpoint};
use xpx_chain_sdk::namespace::NamespaceId;

#[tokio::main]
async fn main() {
    let ep1 = Endpoint::from_shared("http://api-2.testnet2.xpxsirius.io:3000").unwrap();
    let ep2 = Endpoint::from_shared("http://api-3.testnet2.xpxsirius.io:3000").unwrap();

    let endpoints = [ep1, ep2];

    let client = Client::balance_list(endpoints.into_iter());

    let namespace_api = client.namespace_api();

    let namespace_id_one = NamespaceId::try_from_hex("BFFB42A19116BDF6").unwrap();
    let namespace_id_two = NamespaceId::try_from_hex("CCD875904F024AAC").unwrap();

    // Create an Address.
    let address_one = Address::from_raw("VCMCJPRMJ6IUBOZ7HCYBQOSEOVGISX6AMUJ4ESTN").unwrap();

    // Create an Address.
    let address_two = Address::from_raw("VB2HPZ-FL3IRV-62FGZM-7GGIZS-LZGAZE-DZ23P3-EFOR").unwrap();

    // Get the node information.
    let info = namespace_api.get_namespace_info(namespace_id_one).await;
    match info {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!(
            "Exception when calling namespace_api->get_namespace_info: {}\n",
            err
        ),
    }

    // Get namespaces owned by an account.
    let info = namespace_api
        .get_namespaces_from_account(address_one, None)
        .await;
    match info {
        Ok(resp) => println!("{:#?}", resp),
        Err(err) => eprintln!(
            "Exception when calling namespace_api->get_namespaces_from_account: {}\n",
            err
        ),
    }

    // Get namespaces owned by an account.
    let info = namespace_api
        .get_namespaces_from_accounts(vec![address_one, address_two], None)
        .await;
    match info {
        Ok(resp) => println!("{:#?}", resp),
        Err(err) => eprintln!(
            "Exception when calling namespace_api->get_namespaces_from_account: {}\n",
            err
        ),
    }

    // Get namespaces owned by an account.
    let info = namespace_api
        .get_namespaces_names(vec![namespace_id_one, namespace_id_two])
        .await;
    match info {
        Ok(resp) => println!("{:#?}", resp),
        Err(err) => eprintln!(
            "Exception when calling namespace_api->get_namespaces_names: {}\n",
            err
        ),
    }
}
