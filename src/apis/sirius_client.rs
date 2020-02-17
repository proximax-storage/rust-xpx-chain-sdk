use std::fmt::Debug;
use std::sync::Arc;

use hyper::client::connect::Connect;

use crate::apis::chain_routes_api::ChainRoutesApiClient;
use crate::apis::node_routes_api::NodeRoutesApiClient;

use super::account_routes_api::AccountRoutesApiClient;
use super::block_routes_api::BlockRoutesApiClient;

#[derive(Clone)]
pub struct SiriusClient<C: hyper::client::connect::Connect> {
    pub account: Box<AccountRoutesApiClient<C>>,
    pub block: Box<BlockRoutesApiClient<C>>,
    pub chain: Box<ChainRoutesApiClient<C>>,
    pub node: Box<NodeRoutesApiClient<C>>,
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
            block: Box::new(BlockRoutesApiClient::new(rc.clone())),
            chain: Box::new(ChainRoutesApiClient::new(rc.clone())),
            node: Box::new(NodeRoutesApiClient::new(rc.clone())),
        })
    }
}

#[derive(Debug)]
pub struct ApiClient<C: hyper::client::connect::Connect> {
    pub base_path: &'static str,
    pub client: hyper::client::Client<C>,
    pub user_agent: Option<String>,
}

impl<C: hyper::client::connect::Connect> ApiClient<C>
    where
        C: Send + Sync,
{
    pub fn from_url(url: &'static str, client: hyper::client::Client<C>) -> Self
        where
            C: Connect + Clone,
    {
        ApiClient {
            base_path: url,
            client,
            user_agent: Some("Sirius/0.0.1/rust".to_owned()),
        }
    }
}
