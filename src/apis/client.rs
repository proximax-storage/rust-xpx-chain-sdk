use std::borrow::Borrow;
use std::fmt::Debug;
use std::sync::Arc;

use hyper::client;

use super::account_routes_api::AccountRoutesApiClient;
use super::configuration::ApiClient;

#[derive(Clone)]
pub struct SiriusClient<C: hyper::client::connect::Connect> {
    pub account: Box<AccountRoutesApiClient<C>>,
}

impl<C: hyper::client::connect::Connect> SiriusClient<C> where
    C: Clone + Send + Sync + Debug + 'static
{
    pub fn new(url: &'static str, client: hyper::client::Client<C>) -> Box<Self>
        where
            C: Clone + Send + Sync + Debug + 'static
    {
        let sirius = ApiClient::from_url(url, client);

        let rc = Arc::new(sirius);

        Box::new(SiriusClient {
            account: Box::new(AccountRoutesApiClient::new(rc.clone())),
        })
    }
}
