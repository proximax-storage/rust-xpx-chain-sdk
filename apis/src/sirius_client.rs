use ::std::sync::Arc;

use reqwest::Client as ReqwestClient;

use sdk::{network::NetworkType, transaction::Hash};

use crate::routes::{
    account_routes_api::AccountRoutes, block_routes_api::BlockRoutes,
    chain_routes_api::ChainRoutes, exchange_routes_api::ExchangeRoutes,
    mosaic_routes_api::MosaicRoutes, namespace_routes_api::NamespaceRoutes,
    node_routes_api::NodeRoutes, transaction_routes_api::TransactionRoutes,
};

#[derive(Clone, Serialize)]
pub struct SiriusClient {
    generation_hash: Hash,
    network_type: NetworkType,
    #[serde(skip_serializing)]
    account: Box<AccountRoutes>,
    #[serde(skip_serializing)]
    block: Box<BlockRoutes>,
    #[serde(skip_serializing)]
    chain: Box<ChainRoutes>,
    #[serde(skip_serializing)]
    exchange: Box<ExchangeRoutes>,
    #[serde(skip_serializing)]
    node: Box<NodeRoutes>,
    #[serde(skip_serializing)]
    mosaic: Box<MosaicRoutes>,
    #[serde(skip_serializing)]
    namespace: Box<NamespaceRoutes>,
    #[serde(skip_serializing)]
    transaction: Box<TransactionRoutes>,
}

impl SiriusClient {
    pub fn account_api(&self) -> Box<AccountRoutes> {
        self.account.to_owned()
    }

    pub fn block_api(&self) -> Box<BlockRoutes> {
        self.block.to_owned()
    }

    pub fn chain_api(&self) -> Box<ChainRoutes> {
        self.chain.to_owned()
    }

    pub fn exchange_api(&self) -> Box<ExchangeRoutes> {
        self.exchange.to_owned()
    }

    pub fn node_api(&self) -> Box<NodeRoutes> {
        self.node.to_owned()
    }

    pub fn mosaic_api(&self) -> Box<MosaicRoutes> {
        self.mosaic.to_owned()
    }

    pub fn namespace_api(&self) -> Box<NamespaceRoutes> {
        self.namespace.to_owned()
    }

    pub fn transaction_api(&self) -> Box<TransactionRoutes> {
        self.transaction.to_owned()
    }
}

impl SiriusClient
{
    fn __internal(url: &'static str) -> Box<Self> {
        let api_client = ApiClient::from_url(url);

        let rc = Arc::new(api_client);

        Box::new(SiriusClient {
            generation_hash: "".to_string(),
            network_type: Default::default(),
            account: Box::new(AccountRoutes::new(rc.to_owned())),
            block: Box::new(BlockRoutes::new(rc.to_owned())),
            chain: Box::new(ChainRoutes::new(rc.to_owned())),
            exchange: Box::new(ExchangeRoutes::new(rc.to_owned())),
            node: Box::new(NodeRoutes::new(rc.to_owned())),
            mosaic: Box::new(MosaicRoutes::new(rc.to_owned())),
            namespace: Box::new(NamespaceRoutes::new(rc.to_owned())),
            transaction: Box::new(TransactionRoutes::new(rc.to_owned())),
        })
    }

    async fn __generation_info(&mut self) -> super::Result<()> {
        let block_info = self.block_api().get_block_by_height(1).await;
        match block_info {
            Ok(info) => {
                self.generation_hash = info.generation_hash;
                self.network_type = info.network_type;

                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub async fn new(url: &'static str) -> super::Result<Box<Self>> {
        let mut api = Self::__internal(url);
        api.__generation_info().await?;

        Ok(api)
    }

    pub fn generation_hash(&self) -> String {
        self.generation_hash.to_string()
    }

    pub fn network_type(&self) -> NetworkType {
        self.network_type
    }
}

impl core::fmt::Display for SiriusClient {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f, "{}", serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}

#[derive(Clone)]
pub struct ApiClient {
    pub base_path: &'static str,
    pub client: ReqwestClient,
    pub user_agent: Option<String>,
    pub network_type_id: u8,
}

impl ApiClient
{
    pub fn from_url(url: &'static str) -> Self {
        let client = ReqwestClient::new();
        ApiClient {
            base_path: url,
            client,
            user_agent: Some("Sirius/0.0.1/rust".to_owned()),
            network_type_id: 0
        }
    }
}

