use ::std::sync::Arc;

use reqwest::Method;

use sdk::{account::PublicAccount, exchange::UserExchangeInfo};

use crate::{
    dtos::ExchangeInfoDto, request as __internal_request, Result, sirius_client::ApiClient,
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
}
