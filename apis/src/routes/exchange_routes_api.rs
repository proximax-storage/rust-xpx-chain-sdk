use ::std::sync::Arc;

use reqwest::Method;

use sdk::{
    account::PublicAccount,
    network::NetworkType,

    exchange::{UserExchangeInfo, OfferType, OfferInfo, OfferInfos},
    AssetId
};

use crate::{
    dtos::{ExchangeInfoDto, OfferInfoDTOs}, request as __internal_request, Result, sirius_client::ApiClient,
};

use super::{EXCHANGE_ROUTE, OFFERS_BY_MOSAIC_ROUTE};

/// Node ApiClient routes.
///
#[derive(Clone)]
pub struct ExchangeRoutes(Arc<ApiClient>);

/// Exchange related endpoints.
///
impl ExchangeRoutes
{
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        ExchangeRoutes(client)
    }

    fn __client(self) -> Arc<ApiClient> {
        self.0.to_owned()
    }

    pub async fn get_account_exchange_info(
        self,
        account: PublicAccount,
    ) -> Result<UserExchangeInfo> {
        let mut req = __internal_request::Request::new(Method::GET, EXCHANGE_ROUTE.to_string());

        req = req.with_path_param("accountId".to_string(), account.public_key.to_string());

        let dto: Result<ExchangeInfoDto> = req.execute(self.__client()).await;

        Ok(dto?.to_struct(account.address.network_type)?)
    }

    pub async fn get_exchange_offer_by_asset_id(
        self,
        asset_id: impl AssetId, offer_type: OfferType
    ) -> Result<OfferInfos> {
        let mut req = __internal_request::Request::new(Method::GET, OFFERS_BY_MOSAIC_ROUTE.to_string());

        // req = req.with_path_param("mosaic_id".to_string(), asset_id.to_string());

        req = req.with_path_param("offerType".to_string(), offer_type.to_string());

        let dto: OfferInfoDTOs = req.execute(self.__client()).await?;

        let  network_type = NetworkType::from(168);

        let mut offer_infos: OfferInfos = vec![];
            for offer_info_dto in dto.iter(){
                // let offer = offer_info_dto.to_struct(network_type.to_owned())?;
                // offer_infos.push(offer);
            }

        Ok(offer_infos)
    }
}
