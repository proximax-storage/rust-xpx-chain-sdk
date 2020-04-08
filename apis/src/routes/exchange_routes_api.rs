use ::std::{fmt::Debug, sync::Arc};

use hyper::{client::connect::Connect, Method};

use sdk::{account::PublicAccount, exchange::UserExchangeInfo};

use crate::dtos::ExchangeInfoDto;

use super::{EXCHANGE_ROUTE, OFFERS_BY_MOSAIC_ROUTE};
use crate::{request as __internal_request, sirius_client::ApiClient, Result};

/// Node ApiClient routes.
///
#[derive(Clone)]
pub struct ExchangeRoutes<C: Connect>(Arc<ApiClient<C>>);

/// Exchange related endpoints.
///
impl<C: Connect> ExchangeRoutes<C>
where
    C: Clone + Send + Sync + Debug + 'static,
{
    pub(crate) fn new(client: Arc<ApiClient<C>>) -> Self {
        ExchangeRoutes(client)
    }

    fn __client(self) -> Arc<ApiClient<C>> {
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
