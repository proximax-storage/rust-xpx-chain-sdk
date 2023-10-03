/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::api::OrderV2;
use crate::metadata::MetadataV2Type;
use crate::mosaic::UnresolvedMosaicId;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MetadataSortingField(&'static str);

#[derive(Clone, Debug, PartialEq, Builder, Serialize)]
#[builder(setter(into), create_empty = "empty", build_fn(error = "crate::api::error::Error"))]
#[serde(rename_all = "camelCase")]
pub struct MetadataQueryParams {
	#[serde(skip_serializing_if = "Option::is_none")]
	#[builder(default)]
	pub(crate) page_size: Option<u16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[builder(default)]
	pub(crate) page_number: Option<u16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[builder(default)]
	pub(crate) metadata_type: Option<MetadataV2Type>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[builder(default)]
	pub(crate) order: Option<OrderV2>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[builder(default)]
	pub(crate) sort_field: Option<MetadataSortingField>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[builder(default)]
	pub(crate) source_address: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[builder(default)]
	pub(crate) target_id: Option<Box<dyn UnresolvedMosaicId>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[builder(default)]
	pub(crate) target_key: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[builder(default)]
	pub(crate) scoped_metadata_key: Option<String>,
}

impl MetadataQueryParams {
	/// Build a `MetadataQueryParams` from the defaults
	#[must_use]
	pub fn builder() -> MetadataQueryParamsBuilder {
		MetadataQueryParamsBuilder::default()
	}

	/// Serializes a `MetadataQueryParams` into a query string.
	///
	pub fn to_query_string(&self) -> String {
		qs::to_string(self).unwrap()
	}
}

impl TryFrom<&mut MetadataQueryParamsBuilder> for MetadataQueryParams {
	type Error = crate::api::error::Error;

	fn try_from(builder: &mut MetadataQueryParamsBuilder) -> Result<Self, Self::Error> {
		builder.build()
	}
}
