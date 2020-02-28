use ::std::fmt::Debug;
use ::std::sync::Arc;

use hyper::{Client, client::connect::Connect};

use super::{
    account_routes_api::AccountRoutesApiClient,
    block_routes_api::BlockRoutesApiClient,
    chain_routes_api::ChainRoutesApiClient,
    mosaic_routes_api::MosaicRoutesApiClient,
    node_routes_api::NodeRoutesApiClient,
    transaction_routes_api::TransactionRoutesApiClient,
};
use std::future::Future;

#[derive(Clone)]
pub struct SiriusClient<C: Connect> {
    generation_hash: &'static str,
    pub account: Box<AccountRoutesApiClient<C>>,
    pub block: Box<BlockRoutesApiClient<C>>,
    pub chain: Box<ChainRoutesApiClient<C>>,
    pub node: Box<NodeRoutesApiClient<C>>,
    pub mosaic: Box<MosaicRoutesApiClient<C>>,
    pub transaction: Box<TransactionRoutesApiClient<C>>,
}

impl<C: Connect> SiriusClient<C> where
    C: Clone + Send + Sync + Debug + 'static
{
    pub fn new(url: &'static str, client: Client<C>) -> Box<Self>
    {
        let sirius = ApiClient::from_url(url, client);

        let rc = Arc::new(sirius);

        Box::new(SiriusClient {
            generation_hash: "",
            account: Box::new(AccountRoutesApiClient::new(rc.clone())),
            block: Box::new(BlockRoutesApiClient::new(rc.clone())),
            chain: Box::new(ChainRoutesApiClient::new(rc.clone())),
            node: Box::new(NodeRoutesApiClient::new(rc.clone())),
            mosaic: Box::new(MosaicRoutesApiClient::new(rc.clone())),
            transaction: Box::new(TransactionRoutesApiClient::new(rc.clone())),
        })
    }

    pub fn generation_hash(&self) -> impl Future<Output = String> + '_ {
        let client = self.block.clone();
        async {
            let block_info = client.get_block_by_height(1).await;
            match block_info {
                Ok(hash) =>  hash.generation_hash,
                _ => "".to_string()
            }
        }
    }
}

#[derive(Clone)]
pub struct ApiClient<C: Connect> {
    pub base_path: &'static str,
    pub client: Client<C>,
    pub user_agent: Option<String>,
}

impl<C: Connect> ApiClient<C>
    where
        C: Send + Sync,
{
    pub fn from_url(url: &'static str, client: Client<C>) -> Self
    {
        ApiClient {
            base_path: url,
            client,
            user_agent: Some("Sirius/0.0.1/rust".to_owned()),
        }
    }
}
