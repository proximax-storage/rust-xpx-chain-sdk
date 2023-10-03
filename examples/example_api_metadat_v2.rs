/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::time::Duration;

use xpx_chain_sdk::api::{ApiNode, MetadataQueryParams};
use xpx_chain_sdk::metadata::MetadataV2Type;
use xpx_chain_sdk::AsUint64;

#[tokio::main]
async fn main() -> Result<(), xpx_chain_sdk::api::error::Error> {
	let client = ApiNode::builder()
		.uri("http://bctestnet3.brimstone.xpxsirius.io:3000")
		// .timeout(Duration::from_secs(1))
		.connect_timeout(Duration::from_secs(5))
		.build()?
		.connect()
		.await?;

	let metadata_v2_api = client.metadata_v2_api();

	// Get the node information.
	let result = metadata_v2_api
		.get_metadata_v2_info("650E314F4A497F3FF37669FBD5D8E50EEDDBB3430C5C27B453214A9B1D43288C")
		.await;
	match result {
		Ok(resp) => println!("{}", resp),
		Err(err) => {
			eprintln!("Exception when calling metadata_v2_api->get_metadata_v2_info: {}\n", err)
		},
	}

	let result = metadata_v2_api
		.get_metadatas_v2_info(vec![
			"D54146ABED5E29D1F5B1D7FDAB36C807AD7B82B3D8E2E251DC32F84CE3E56730",
			"544F408D38BB884193241E32EE19D9870D332B1C38FBDFA9DF9BC7650F02FD38",
		])
		.await;
	match result {
		Ok(resp) => println!("{:#?}", resp),
		Err(err) => {
			eprintln!("Exception when calling metadata_v2_api->get_metadatas_v2_info: {}\n", err)
		},
	}

	let scoped_key = format!("{:X}", u64::from_utf8_str("my_key_text"));

	let query_params: MetadataQueryParams = MetadataQueryParams::builder()
		.metadata_type(MetadataV2Type::MetadataAccount)
		.page_size(Some(1_u16))
		.target_key("D14E8C9759FD194BD3BCAD5AD7228F2B4BEA5DF03ED63516EEFDAC3CD30128DE".to_string())
		.scoped_metadata_key(Some(scoped_key))
		.try_into()?;

	let result = metadata_v2_api.search_metadata(Some(query_params)).await;
	match result {
		Ok(resp) => println!("{:#?}", resp),
		Err(err) => eprintln!("Exception when calling metadata_v2_api->search_metadata: {}\n", err),
	}
	Ok(())
}
