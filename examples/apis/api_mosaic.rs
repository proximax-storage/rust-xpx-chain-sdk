// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.
#![deny(warnings)]
#![warn(rust_2018_idioms)]

use xpx_chain_apis::SiriusClient;
use xpx_chain_sdk::AssetId;
use xpx_chain_sdk::mosaic::MosaicId;

#[tokio::main]
async fn main() {
    let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];

    let sirius_client = SiriusClient::new(node_url).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let mosaic_one = MosaicId::from_hex("3C520B7CEB2F7099").unwrap();

    let mosaic_info = client
        .mosaic_api()
        .get_mosaic_info(mosaic_one)
        .await;

    match mosaic_info {
        Ok(resp) => println!("{}", resp),
        Err(err) => panic!("{}", err),
    }

    let mosaic_two = MosaicId::from_hex("13bfc518e40549d7").unwrap();
    let mosaic_three = MosaicId::from_hex("6208AE4D56451357").unwrap();

    println!("{:?}", mosaic_three.to_u32_array());
    let mosaics_info = client
        .mosaic_api()
        .get_mosaics_info(vec![mosaic_two.clone(), mosaic_three.clone()])
        .await;

    match mosaics_info {
        Ok(mosaics) => mosaics
            .iter()
            .for_each(|mosaic_info| println!("{}", mosaic_info)),
        Err(err) => panic!("{}", err),
    }

    let mosaics_names = client
        .mosaic_api()
        .get_mosaics_names(vec![mosaic_two, mosaic_three])
        .await;

    match mosaics_names {
        Ok(mosaic) => mosaic.iter().for_each(|name| println!("{}", name)),
        Err(err) => panic!("{}", err),
    }
}
