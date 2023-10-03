/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use hyper::Method;
use serde_json::json;

use crate::api::routes::const_routes::{METADATAS_V2_INFO_ROUTE, METADATA_V2_INFO_ROUTE};
use crate::api::transport::service::Connection;
use crate::api::{MetadataQueryParams, MetadataV2InfoDTO, SearchMetadataV2InfoDTO};
use crate::metadata::{MetadataEntry, SearchMetadataEntry};
use crate::{api::error::Result, api::request as __internal_request};

/// Namespace ApiClient routes.
///
#[derive(Clone)]
pub struct MetadataV2Routes(Connection);

/// Account related endpoints.
///
impl MetadataV2Routes {
	pub(crate) fn new(client: Connection) -> Self {
		MetadataV2Routes(client)
	}

	fn __client(&self) -> Connection {
		self.0.clone()
	}

	pub async fn get_metadata_v2_info(&self, composite_hash: &str) -> Result<MetadataEntry> {
		let mut req =
			__internal_request::ApiRequest::new(Method::GET, METADATA_V2_INFO_ROUTE.to_string());

		req = req.with_path_param("compositeHash".to_string(), composite_hash.to_string());

		let dto: MetadataV2InfoDTO = req.execute(self.__client()).await?;

		Ok(dto.compact()?)
	}

	pub async fn get_metadatas_v2_info(
		&self,
		composite_hashes: Vec<&str>,
	) -> Result<Vec<MetadataEntry>> {
		let body_param = json!({
			"compositeHashes": composite_hashes,
		});

		let mut req =
			__internal_request::ApiRequest::new(Method::POST, METADATAS_V2_INFO_ROUTE.to_string());

		req = req.with_body_param(body_param);

		let dto: Vec<MetadataV2InfoDTO> = req.execute(self.__client()).await?;

		let mut metadatas_entry = vec![];
		for info_dto in dto.into_iter() {
			metadatas_entry.push(info_dto.compact()?);
		}

		Ok(metadatas_entry)
	}

	pub async fn search_metadata(
		&self,
		query_params: Option<impl Into<MetadataQueryParams>>,
	) -> Result<SearchMetadataEntry> {
		let mut req =
			__internal_request::ApiRequest::new(Method::GET, METADATAS_V2_INFO_ROUTE.to_string());

		if let Some(params) = query_params {
			let params = params.into();

			println!("{}", params.to_query_string());
			req = req.with_query_param(params.to_query_string());
		}

		let dto: SearchMetadataV2InfoDTO = req.execute(self.__client()).await?;

		Ok(dto.compact()?)
	}
}
