/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::time::Duration;

use xpx_chain_sdk::api::ApiNode;
use xpx_chain_sdk::mosaic::{MosaicId, UnresolvedMosaicId};
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

    let resolver_api = client.resolver_api();

    let asset_id_one = NamespaceId::try_from_hex("BFFB42A19116BDF6")?;
    let asset_id_two = MosaicId::try_from_hex("13BFC518E40549D7")?;

    let info = resolver_api.get_mosaic_info_by_asset_id(asset_id_one.box_clone()).await;
    match info {
        Ok(resp) => println!("{}", resp),
        Err(err) => {
            eprintln!("Exception when calling resolver_api->get_mosaic_info_by_asset_id: {}\n", err)
        }
    }

    let info = resolver_api
        .get_mosaic_info_by_asset_ids(vec![asset_id_one.box_clone(), asset_id_two.box_clone()])
        .await;
    match info {
        Ok(resp) => println!("{:#?}", resp),
        Err(err) => eprintln!(
            "Exception when calling resolver_api->get_mosaic_info_by_asset_ids: {}\n",
            err
        ),
    }

    Ok(())
}
