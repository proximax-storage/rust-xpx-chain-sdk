/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {::std::sync::Arc, reqwest::Client as ReqwestClient};

use crate::{models::error::Result, network::NetworkType, transaction::HashValue};

use super::routes::{
    account_routes_api::AccountRoutes, block_routes_api::BlockRoutes,
    chain_routes_api::ChainRoutes, exchange_routes_api::ExchangeRoutes,
    metadata_routes_api::MetadataRoutes, mosaic_routes_api::MosaicRoutes,
    namespace_routes_api::NamespaceRoutes, node_routes_api::NodeRoutes,
    resolver_routes_api::ResolverRoutes, transaction_routes_api::TransactionRoutes,
};

#[derive(Clone, Serialize)]
pub struct SiriusClient {
    generation_hash: HashValue,
    network_type: NetworkType,
    #[serde(skip_serializing)]
    client: Arc<ApiClient>,
}

impl SiriusClient {
    fn __client(&self) -> Arc<ApiClient> {
        Arc::clone(&self.client)
    }

    pub fn account_api(&self) -> Box<AccountRoutes> {
        Box::new(AccountRoutes::new(self.__client()))
    }

    pub fn block_api(&self) -> Box<BlockRoutes> {
        Box::new(BlockRoutes::new(self.__client()))
    }

    pub fn chain_api(&self) -> Box<ChainRoutes> {
        Box::new(ChainRoutes::new(self.__client()))
    }

    pub fn exchange_api(&self) -> Box<ExchangeRoutes> {
        Box::new(ExchangeRoutes::new(
            self.__client(),
            self.network_type(),
            *self.resolver_api(),
        ))
    }

    pub fn node_api(&self) -> Box<NodeRoutes> {
        Box::new(NodeRoutes::new(self.__client()))
    }

    pub fn mosaic_api(&self) -> Box<MosaicRoutes> {
        Box::new(MosaicRoutes::new(self.__client()))
    }

    pub fn namespace_api(&self) -> Box<NamespaceRoutes> {
        Box::new(NamespaceRoutes::new(self.__client(), self.network_type()))
    }

    pub fn transaction_api(&self) -> Box<TransactionRoutes> {
        Box::new(TransactionRoutes::new(self.__client()))
    }

    pub fn resolver_api(&self) -> Box<ResolverRoutes> {
        Box::new(ResolverRoutes::new(
            self.__client(),
            *self.namespace_api(),
            *self.mosaic_api(),
        ))
    }

    pub fn metadata_api(&self) -> Box<MetadataRoutes> {
        Box::new(MetadataRoutes::new(self.__client()))
    }
}

impl SiriusClient {
    fn __internal(url_node: String) -> Box<Self> {
        let api_client = ApiClient::from_url(url_node);

        let client = Arc::new(api_client);

        Box::new(SiriusClient {
            generation_hash: HashValue::zero(),
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

    pub async fn new<T: AsRef<str>>(urls: &[T]) -> Result<Box<Self>> {
        //TODO
        let url_node = String::from(urls[0].as_ref());

        let mut api = Self::__internal(url_node);
        api.__generation_info().await?;

        Ok(api)
    }

    pub fn generation_hash(&self) -> HashValue {
        self.generation_hash
    }

    pub fn network_type(&self) -> NetworkType {
        self.network_type
    }

    pub fn node(&self) -> &str {
        &self.client.base_path
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

pub(crate) struct ApiClient {
    pub base_path: String,
    pub client: ReqwestClient,
    pub user_agent: Option<String>,
}

impl ApiClient {
    pub fn from_url(url: String) -> Self {
        let client = ReqwestClient::new();
        ApiClient {
            base_path: url,
            client,
            user_agent: Some("Sirius/0.0.1/rust".to_owned()),
        }
    }
}
