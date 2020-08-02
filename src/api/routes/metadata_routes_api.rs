/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {::std::sync::Arc, reqwest::Method};

use crate::{
    account::Address,
    api::{
        internally::valid_vec_len,
        request as __internal_request,
        routes::const_routes::{
            METADATA_BY_ACCOUNT_ROUTE, METADATA_BY_MOSAIC_ROUTE, METADATA_BY_NAMESPACE_ROUTE,
            METADATA_INFO_ROUTE,
        },
        sirius_client::ApiClient,
        AddressMetadataInfoDto, MetadataDto, MosaicMetadataInfoDto, NamespaceMetadataInfoDto,
    },
    errors_const::{ERR_METADATA_EMPTY_ADDRESSES, ERR_METADATA_EMPTY_MOSAIC_IDS},
    metadata::{AddressMetadataInfo, MetadataIds, MosaicMetadataInfo, NamespaceMetadataInfo},
    models::Result,
    mosaic::MosaicId,
    namespace::NamespaceId,
    AssetId,
};

/// Namespace ApiClient routes.
///
#[derive(Clone)]
pub struct MetadataRoutes(Arc<ApiClient>);

/// Account related endpoints.
///
impl MetadataRoutes {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        MetadataRoutes(client)
    }

    fn __client(&self) -> Arc<ApiClient> {
        Arc::clone(&self.0)
    }

    pub async fn get_metadata_by_address(self, address: Address) -> Result<AddressMetadataInfo> {
        let mut req =
            __internal_request::Request::new(Method::GET, METADATA_BY_ACCOUNT_ROUTE.to_string());

        req = req.with_path_param("address_id".to_string(), address.address_string());

        let dto: MetadataDto<AddressMetadataInfoDto> = req.execute(self.__client()).await?;

        Ok(dto.metadata.compact()?)
    }

    pub async fn get_metadata_by_mosaic_id(
        self,
        mosaic_id: MosaicId,
    ) -> Result<MosaicMetadataInfo> {
        let mut req =
            __internal_request::Request::new(Method::GET, METADATA_BY_MOSAIC_ROUTE.to_string());

        req = req.with_path_param("mosaic_id".to_string(), mosaic_id.to_hex());

        let dto: MetadataDto<MosaicMetadataInfoDto> = req.execute(self.__client()).await?;

        Ok(dto.metadata.compact()?)
    }

    pub async fn get_metadata_by_namespace_id(
        self,
        namespace_id: NamespaceId,
    ) -> Result<NamespaceMetadataInfo> {
        let mut req =
            __internal_request::Request::new(Method::GET, METADATA_BY_NAMESPACE_ROUTE.to_string());

        req = req.with_path_param("namespace_id".to_string(), namespace_id.to_hex());

        let dto: MetadataDto<NamespaceMetadataInfoDto> = req.execute(self.__client()).await?;

        Ok(dto.metadata.compact()?)
    }

    pub async fn get_addresses_metadata(
        self,
        addresses: Vec<&str>,
    ) -> Result<Vec<AddressMetadataInfo>> {
        valid_vec_len(&addresses, ERR_METADATA_EMPTY_ADDRESSES)?;

        let addresses = MetadataIds::from(addresses);

        let mut req =
            __internal_request::Request::new(Method::POST, METADATA_INFO_ROUTE.to_string());

        req = req.with_body_param(&addresses);

        let dto: Vec<MetadataDto<AddressMetadataInfoDto>> = req.execute(self.__client()).await?;

        let mut address_info: Vec<AddressMetadataInfo> = vec![];
        for address_dto in dto.into_iter() {
            address_info.push(address_dto.metadata.compact()?);
        }

        Ok(address_info)
    }

    pub async fn get_mosaics_id_metadata(
        self,
        mosaic_ids: Vec<MosaicId>,
    ) -> Result<Vec<MosaicMetadataInfo>> {
        valid_vec_len(&mosaic_ids, ERR_METADATA_EMPTY_MOSAIC_IDS)?;

        let mosaic_ids = MetadataIds::from(mosaic_ids);

        let mut req =
            __internal_request::Request::new(Method::POST, METADATA_INFO_ROUTE.to_string());

        req = req.with_body_param(&mosaic_ids);

        let dto: Vec<MetadataDto<MosaicMetadataInfoDto>> = req.execute(self.__client()).await?;

        let mut mosaic_info: Vec<MosaicMetadataInfo> = vec![];
        for mosaic_dto in dto.into_iter() {
            mosaic_info.push(mosaic_dto.metadata.compact()?);
        }

        Ok(mosaic_info)
    }

    pub async fn get_namespaces_id_metadata(
        self,
        namespace_ids: Vec<NamespaceId>,
    ) -> Result<Vec<NamespaceMetadataInfo>> {
        valid_vec_len(&namespace_ids, METADATA_BY_NAMESPACE_ROUTE)?;

        let namespace_ids = MetadataIds::from(namespace_ids);

        let mut req =
            __internal_request::Request::new(Method::POST, METADATA_INFO_ROUTE.to_string());

        req = req.with_body_param(&namespace_ids);

        let dto: Vec<MetadataDto<NamespaceMetadataInfoDto>> = req.execute(self.__client()).await?;

        let mut namespace_info: Vec<NamespaceMetadataInfo> = vec![];
        for namespace_dto in dto.into_iter() {
            namespace_info.push(namespace_dto.metadata.compact()?);
        }

        Ok(namespace_info)
    }
}
