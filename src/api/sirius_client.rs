/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {::std::sync::Arc, reqwest::Client as ReqwestClient};

use crate::{models::error::Result, network::NetworkType, transaction::Hash};

use super::routes::{
    account_routes_api::AccountRoutes, block_routes_api::BlockRoutes,
    chain_routes_api::ChainRoutes, exchange_routes_api::ExchangeRoutes,
    mosaic_routes_api::MosaicRoutes, namespace_routes_api::NamespaceRoutes,
    node_routes_api::NodeRoutes, resolver_routes_api::ResolverRoutes,
    transaction_routes_api::TransactionRoutes,
};

#[derive(Clone, Serialize)]
pub struct SiriusClient {
    generation_hash: Hash,
    network_type: NetworkType,
    #[serde(skip_serializing)]
    client: Arc<ApiClient>,
}

impl SiriusClient {
    pub fn account_api(&self) -> Box<AccountRoutes> {
        Box::new(AccountRoutes::new(self.client.to_owned()))
    }

    pub fn block_api(&self) -> Box<BlockRoutes> {
        Box::new(BlockRoutes::new(self.client.to_owned()))
    }

    pub fn chain_api(&self) -> Box<ChainRoutes> {
        Box::new(ChainRoutes::new(self.client.to_owned()))
    }

    pub fn exchange_api(&self) -> Box<ExchangeRoutes> {
        Box::new(ExchangeRoutes::new(
            self.client.to_owned(),
            self.network_type(),
            *self.resolver_api(),
        ))
    }

    pub fn node_api(&self) -> Box<NodeRoutes> {
        Box::new(NodeRoutes::new(self.client.to_owned()))
    }

    pub fn mosaic_api(&self) -> Box<MosaicRoutes> {
        Box::new(MosaicRoutes::new(self.client.to_owned()))
    }

    pub fn namespace_api(&self) -> Box<NamespaceRoutes> {
        Box::new(NamespaceRoutes::new(self.client.to_owned()))
    }

    pub fn transaction_api(&self) -> Box<TransactionRoutes> {
        Box::new(TransactionRoutes::new(self.client.to_owned()))
    }

    pub fn resolver_api(&self) -> Box<ResolverRoutes> {
        Box::new(ResolverRoutes::new(
            self.client.to_owned(),
            *self.namespace_api(),
            *self.mosaic_api(),
        ))
    }
}

impl SiriusClient {
    fn __internal(url_node: &'static str) -> Box<Self> {
        let api_client = ApiClient::from_url(url_node);

        let client = Arc::new(api_client);

        Box::new(SiriusClient {
            generation_hash: "".to_string(),
            network_type: Default::default(),
            client,
        })
    }

    async fn __generation_info(&mut self) -> Result<()> {
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

    pub async fn new(urls: Vec<&'static str>) -> Result<Box<Self>> {
        //TODO
        let url_node = urls[0];

        let mut api = Self::__internal(url_node);
        api.__generation_info().await?;

        Ok(api)
    }

    pub fn generation_hash(&self) -> &str {
        &self.generation_hash
    }

    pub fn network_type(&self) -> NetworkType {
        self.network_type
    }

    pub fn node(&self) -> &str {
        self.client.base_path
    }
}

impl core::fmt::Display for SiriusClient {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}

#[derive(Clone)]
pub(crate) struct ApiClient {
    pub base_path: &'static str,
    pub client: ReqwestClient,
    pub user_agent: Option<String>,
}

impl ApiClient {
    pub fn from_url(url: &'static str) -> Self {
        let client = ReqwestClient::new();
        ApiClient {
            base_path: url,
            client,
            user_agent: Some("Sirius/0.0.1/rust".to_owned()),
        }
    }
}
