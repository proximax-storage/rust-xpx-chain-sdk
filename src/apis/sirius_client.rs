use ::std::fmt::Debug;
use ::std::future::Future;
use ::std::sync::Arc;

use hyper::{Client, client::connect::Connect};

use crate::models::{network::NetworkType, transaction::Hash};

use super::{
    account_routes_api::AccountRoutes,
    block_routes_api::BlockRoutes,
    chain_routes_api::ChainRoutes,
    mosaic_routes_api::MosaicRoutes,
    namespace_routes_api::NamespaceRoutes,
    node_routes_api::NodeRoutes,
    transaction_routes_api::TransactionRoutes
};

#[derive(Clone)]
pub struct SiriusClient<C: Connect> {
    generation_hash: Hash,
    pub account: Box<AccountRoutes<C>>,
    pub block: Box<BlockRoutes<C>>,
    pub chain: Box<ChainRoutes<C>>,
    pub node: Box<NodeRoutes<C>>,
    pub mosaic: Box<MosaicRoutes<C>>,
    pub namespace: Box<NamespaceRoutes<C>>,
    pub transaction: Box<TransactionRoutes<C>>,
}

impl<C: Connect> SiriusClient<C> where
    C: Clone + Send + Sync + Debug + 'static
{
    fn __internal(url: &'static str, client: Client<C>) -> Box<Self>
    {
        let api_client = ApiClient::from_url(url, client);

        let rc = Arc::new(api_client);

        Box::new(SiriusClient {
            generation_hash: "".to_string(),
            account: Box::new(AccountRoutes::new(rc.to_owned())),
            block: Box::new(BlockRoutes::new(rc.to_owned())),
            chain: Box::new(ChainRoutes::new(rc.to_owned())),
            node: Box::new(NodeRoutes::new(rc.to_owned())),
            mosaic: Box::new(MosaicRoutes::new(rc.to_owned())),
            namespace: Box::new(NamespaceRoutes::new(rc.to_owned())),
            transaction: Box::new(TransactionRoutes::new(rc.to_owned())),
        })
    }

    pub fn __generation_hash(&self) -> impl Future<Output = super::Result<String>> + '_ {
        let client = self.block.to_owned();
        async {
            let block_info = client.get_block_by_height(1).await;
            match block_info {
                Ok(hash) => Ok(hash.generation_hash),
                Err(err) => Err(err),
            }
        }
    }

    pub async fn new(url: &'static str, client: Client<C>) -> super::Result<Box<Self>> {
        let mut api = Self::__internal(url, client);

        let generation_hash = api.__generation_hash().await?;
        if !generation_hash.is_empty() {
            api.generation_hash = generation_hash
        };
        Ok(api)
    }

    pub fn generation_hash(&self) -> String {
        self.generation_hash.to_string()
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
