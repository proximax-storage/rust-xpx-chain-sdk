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

#[tokio::main]
async fn main() {
    let node_url = "http://bctestnet3.brimstone.xpxsirius.io:3000";

    let sirius_client = SiriusClient::new(node_url).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let mosaic_one = NamespaceId::from_name("prx.xpx").unwrap();

    println!("{}", mosaic_one);
    let mosaic_two = MosaicId::from_hex("24233ae37c9c7e5d").unwrap();

    // println!("------- {} -------", "Get mosaic information");
    // // Gets the mosaic definition for a given mosaicId.
    // let mosaic_info = client.mosaic_api().get_mosaic_info(mosaic_one).await;
    //
    // match mosaic_info {
    //     Ok(resp) => println!("{}\n\n", resp),
    //     Err(err) => eprintln!("sdsfsdfsdf{}", err),
    // }

    // let mosaic_two = MosaicId::from_hex("13bfc518e40549d7").unwrap();
    // let mosaic_three = MosaicId::from_hex("6208AE4D56451357").unwrap();
    //
    // println!(
    //     "------- {} -------",
    //     "Get mosaics information for an array of mosaics "
    // );
    // // Gets an vector of mosaic definition.
    // let mosaics_info = client
    //     .mosaic_api()
    //     .get_mosaics_info(vec![mosaic_two, mosaic_three])
    //     .await;
    //
    // match mosaics_info {
    //     Ok(mosaics) => mosaics
    //         .iter()
    //         .for_each(|mosaic_info| println!("{}", mosaic_info)),
    //     Err(err) => eprintln!("{}", err),
    // }
    //
    // println!(
    //     "------- {} -------",
    //     "Get readable names for a set of mosaics "
    // );
    // // Returns friendly names for mosaics.
    // let mosaics_names = client
    //     .mosaic_api()
    //     .get_mosaics_names(vec![mosaic_one, mosaic_two])
    //     .await;
    //
    // match mosaics_names {
    //     Ok(mosaic) => mosaic.iter().for_each(|name| println!("{}", name)),
    //     Err(err) => eprintln!("{}", err),
    // }
}
