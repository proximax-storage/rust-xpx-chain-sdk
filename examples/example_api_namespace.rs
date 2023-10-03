/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::time::Duration;

use xpx_chain_sdk::account::Address;
use xpx_chain_sdk::api::ApiNode;
use xpx_chain_sdk::namespace::NamespaceId;

#[tokio::main]
async fn main() -> Result<(), xpx_chain_sdk::api::error::Error> {
    let client = ApiNode::builder()
        .uri("http://bctestnet3.brimstone.xpxsirius.io:3000")
        // .timeout(Duration::from_secs(1))
        .connect_timeout(Duration::from_secs(5))
        .build()?
        .connect()
        .await?;

    let namespace_api = client.namespace_api();

    let namespace_id_one = NamespaceId::from_name("prx.xpx")?;
    let namespace_id_two = NamespaceId::try_from_hex("BFFB42A19116BDF6")?;

    // Create an Address.
    let address_one = Address::from_raw("VCMCJPRMJ6IUBOZ7HCYBQOSEOVGISX6AMUJ4ESTN")?;

    // Create an Address.
    let address_two = Address::from_raw("VB2HPZ-FL3IRV-62FGZM-7GGIZS-LZGAZE-DZ23P3-EFOR")?;

    // Get the node information.
    let info = namespace_api.get_namespace_info(namespace_id_one).await;
    match info {
        Ok(resp) => println!("{}", resp),
        Err(err) => {
            eprintln!("Exception when calling namespace_api->get_namespace_info: {}\n", err)
        }
    }

    // Get namespaces owned by an account.
    let info = namespace_api.get_namespaces_from_account(address_one, None).await;
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
        Err(err) => {
            eprintln!("Exception when calling namespace_api->get_namespaces_names: {}\n", err)
        }
    }
    Ok(())
}
