/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

//#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_sdk::api::SiriusClient;
use xpx_chain_sdk::mosaic::MosaicId;
use xpx_chain_sdk::namespace::NamespaceId;
use xpx_chain_sdk::error::{SiriusError, Error};
use std::any::{TypeId, Any};
use xpx_chain_sdk::account::Account;
use xpx_chain_sdk::network::PUBLIC_TEST;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let node_url = "http://bctestnet1.brimstone.xpxsirius.io:3000";

    println!("{:?}", Account::from_private_key("B38A1490B33A4BD718ABB0A1BEF389CAE07A435F3DEC39BC518D84B1ABF8531B", PUBLIC_TEST).unwrap().public_account.public_key);

    let sirius_client = SiriusClient::new(node_url).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => {
            if let Error::Reqwest(_err) = err {
                println!("##################### ERROR: {:?}", _err);
            }
            panic!("")
        },
    };

    // let mosaic_one = MosaicId::from_name("prx.xpx").unwrap();

    let mosaic_two = MosaicId::from_hex("24233ae37c9c7e6d").unwrap();

    println!("{}", hex::encode([241, 218, 170, 74, 249, 54, 5, 85, 105, 89, 231, 71, 159, 219, 188, 218, 163, 232, 130, 48]));

    println!("------- {} -------", "Get mosaic information");
    // Gets the mosaic definition for a given mosaicId.

    let hashes = vec!["3a4e051b7c138b8a843c210df68a3e33ec29bd56879f1611abf5a0bdac422bef".to_string()];
    let mosaic_info = client.block_api().get_block_by_height(1).await;

    match mosaic_info {
        Ok(resp) => println!("{}\n\n", resp),
        Err(ref err) => {
            if let Error::Reqwest(err) = err{
                println!("{}", err)
            }
        },
    }

    // let mosaic_two = MosaicId::from_hex("13bfc518e40549d7").unwrap();
    let mosaic_three = MosaicId::from_hex("6208AE4D56451357").unwrap();

    println!(
        "------- {} -------",
        "Get mosaics information for an array of mosaics "
    );
    // Gets an vector of mosaic definition.
    let mosaics_info = client
        .mosaic_api()
        .get_mosaics_info(vec![mosaic_three])
        .await;

    match mosaics_info {
        Ok(mosaics) => mosaics
            .iter()
            .for_each(|mosaic_info| println!("{}", mosaic_info)),
        Err(err) => eprintln!("{}", err),
    }

    println!(
        "------- {} -------",
        "Get readable names for a set of mosaics "
    );
    // Returns friendly names for mosaics.
    let mosaics_names = client
        .mosaic_api()
        .get_mosaics_names(vec![mosaic_two])
        .await;

    match mosaics_names {
        Ok(mosaic) => mosaic.iter().for_each(|name| println!("{}", name)),
        Err(err) => eprintln!("{}", err),
    }
}