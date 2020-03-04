use ::std::fmt::Debug;
use ::std::future::Future;
use ::std::sync::Arc;

use hyper::{Client, client::connect::Connect};

use crate::models::network::{NetworkType, NOT_SUPPORTED_NET};

use super::{
    account_routes_api::AccountRoutes,
    block_routes_api::BlockRoutes,
    chain_routes_api::ChainRoutes,
    mosaic_routes_api::MosaicRoutes,
    node_routes_api::NodeRoutes,
    transaction_routes_api::TransactionRoutes,
};

#[derive(Clone)]
pub struct SiriusClient<C: Connect> {
    generation_hash: &'static str,
    pub account: Box<AccountRoutes<C>>,
    pub block: Box<BlockRoutes<C>>,
    pub chain: Box<ChainRoutes<C>>,
    pub node: Box<NodeRoutes<C>>,
    pub mosaic: Box<MosaicRoutes<C>>,
    pub transaction: Box<TransactionRoutes<C>>,
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
            account: Box::new(AccountRoutes::new(rc.clone())),
            block: Box::new(BlockRoutes::new(rc.clone())),
            chain: Box::new(ChainRoutes::new(rc.clone())),
            node: Box::new(NodeRoutes::new(rc.clone())),
            mosaic: Box::new(MosaicRoutes::new(rc.clone())),
            transaction: Box::new(TransactionRoutes::new(rc.clone())),
        })
    }

    pub fn generation_hash(&self) -> impl Future<Output = String> + '_ {
        let client = self.block.clone();
        async {
            let block_info = client.get_block_by_height(1).await;
            match block_info {
                Ok(hash) => hash.generation_hash,
                Err(err) => panic!("{:?}", err),
            }
        }
    }

    pub fn network_type(&self) -> impl Future<Output = NetworkType> + '_ {
        let client = self.node.clone();
        async {
            let block_info = client.get_node_info().await;
            match block_info {
                Ok(node) => NetworkType::from(node.network_identifier),
                Err(err) => panic!("{:?}", err),
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
