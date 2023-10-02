/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::time::Duration;

use xpx_chain_sdk::api::{ApiNode, PageQueryParams};
use xpx_chain_sdk::mosaic::MosaicId;

#[tokio::main]
async fn main() -> Result<(), xpx_chain_sdk::api::error::Error> {
    let client = ApiNode::builder()
        .uri("http://bctestnet3.brimstone.xpxsirius.io:3000")
        // .timeout(Duration::from_secs(1))
        .connect_timeout(Duration::from_secs(5))
        .build()?
        .connect()
        .await?;

    let mosaic_api = client.mosaic_api();

    let mosaic_id_one = MosaicId::try_from_hex("13BFC518E40549D7")?;
    let mosaic_id_two = MosaicId::try_from_hex("705BAFA9B6903C08")?;

    // Get the node information.
    let info = mosaic_api.get_mosaic_info(mosaic_id_one).await;
    match info {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("Exception when calling mosaic_api->get_mosaic_info: {}\n", err),
    }

    // Get namespaces owned by an account.
    let info = mosaic_api.get_mosaics_info(vec![mosaic_id_one, mosaic_id_two]).await;
    match info {
        Ok(resp) => println!("{:#?}", resp),
        Err(err) => eprintln!("Exception when calling mosaic_api->get_mosaics_info: {}\n", err),
    }

    // Get namespaces owned by an account.
    let info = mosaic_api.get_mosaics_names(vec![mosaic_id_one, mosaic_id_two]).await;
    match info {
        Ok(resp) => println!("{:#?}", resp),
        Err(err) => eprintln!("Exception when calling mosaic_api->get_mosaics_names: {}\n", err),
    }

    let info = mosaic_api
        .get_mosaic_rich_list(mosaic_id_one, Some(PageQueryParams::create(5, 30)))
        .await;
    match info {
        Ok(resp) => println!("{:#?}", resp.len()),
        Err(err) => eprintln!("Exception when calling mosaic_api->get_mosaic_rich_list: {}\n", err),
    }

    let info = mosaic_api.get_mosaic_levy(mosaic_id_one).await;
    match info {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("Exception when calling mosaic_api->get_mosaic_levy: {}\n", err),
    }
    Ok(())
}
