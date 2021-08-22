/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {::std::sync::Arc, reqwest::Method};

use crate::{
    account::PublicAccount,
    api::{ExchangeInfoDto, request as __internal_request, sirius_client::ApiClient},
    AssetId,
    AssetIdType,
    exchange::{OfferInfos, OfferType, UserExchangeInfo},
    models::Result, network::NetworkType,
};
use crate::api::OfferInfoDTOs;

use super::{EXCHANGE_ROUTE, OFFERS_BY_MOSAIC_ROUTE, resolver_routes_api::ResolverRoutes};

/// Node ApiClient routes.
///
#[derive(Clone)]
pub struct ExchangeRoutes(Arc<ApiClient>, NetworkType, ResolverRoutes);

/// Exchange related endpoints.
///
impl ExchangeRoutes {
    pub(crate) fn new(
        client: Arc<ApiClient>,
        network_type: NetworkType,
        resolver_routes: ResolverRoutes,
    ) -> Self {
        ExchangeRoutes(client, network_type, resolver_routes)
    }

    fn __client(&self) -> Arc<ApiClient> {
        Arc::clone(&self.0)
    }

    fn __network_type(&self) -> NetworkType {
        self.1
    }

    fn __resolver_routes(self) -> ResolverRoutes {
        self.2
    }

    pub async fn get_account_exchange_info(
        self,
        public_account: PublicAccount,
    ) -> Result<UserExchangeInfo> {
        let mut req = __internal_request::Request::new(Method::GET, EXCHANGE_ROUTE.to_string());

        req = req.with_path_param("account_id".to_string(), public_account.public_key_string());

        let dto: Result<ExchangeInfoDto> = req.execute(self.__client()).await;

        Ok(dto?.compact(public_account.address.network_type())?)
    }

    pub async fn get_exchange_offer_by_asset_id(
        self,
        asset_id: impl AssetId,
        offer_type: OfferType,
    ) -> Result<OfferInfos> {
        let mut req =
            __internal_request::Request::new(Method::GET, OFFERS_BY_MOSAIC_ROUTE.to_string());

        let asset_id = match asset_id.get_type() {
            AssetIdType::NamespaceIdType => {
                let asset_info = self
                    .clone()
                    .__resolver_routes()
                    .get_mosaic_info_by_asset_id(asset_id)
                    .await?;
                asset_info.mosaic_id.to_hex()
            }
            _ => asset_id.to_hex(),
        };
        req = req.with_path_param("mosaic_id".to_string(), asset_id);

        req = req.with_path_param("offer_type".to_string(), offer_type.to_string());

        let network_type = self.__network_type();

        let dto: OfferInfoDTOs = req.execute(self.__client()).await?;

        let mut offer_infos: OfferInfos = vec![];
        for offer_info_dto in &dto {
            let owner_account = PublicAccount::from_public_key(
                &offer_info_dto.owner.as_ref().unwrap(),
                network_type,
            )?;

            let offer = offer_info_dto.compact(owner_account)?;
            offer_infos.push(offer);
        }

        Ok(offer_infos)
    }
}
