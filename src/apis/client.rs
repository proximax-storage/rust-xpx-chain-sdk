use hyper;

use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient {
    account_routes_api: Box<dyn crate::apis::AccountRoutesApi>,
    block_routes_api: Box<dyn crate::apis::BlockRoutesApi>,
    chain_routes_api: Box<dyn crate::apis::ChainRoutesApi>,
    config_routes_api: Box<dyn crate::apis::ConfigRoutesApi>,
    contract_routes_api: Box<dyn crate::apis::ContractRoutesApi>,
    diagnostic_routes_api: Box<dyn crate::apis::DiagnosticRoutesApi>,
    metadata_routes_api: Box<dyn crate::apis::MetadataRoutesApi>,
    mosaic_routes_api: Box<dyn crate::apis::MosaicRoutesApi>,
    namespace_routes_api: Box<dyn crate::apis::NamespaceRoutesApi>,
    network_routes_api: Box<dyn crate::apis::NetworkRoutesApi>,
    node_routes_api: Box<dyn crate::apis::NodeRoutesApi>,
    transaction_routes_api: Box<dyn crate::apis::TransactionRoutesApi>,
    upgrade_routes_api: Box<dyn crate::apis::UpgradeRoutesApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::Connect>(configuration: Configuration<C>) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            account_routes_api: Box::new(crate::apis::AccountRoutesApiClient::new(rc.clone())),
            block_routes_api: Box::new(crate::apis::BlockRoutesApiClient::new(rc.clone())),
            chain_routes_api: Box::new(crate::apis::ChainRoutesApiClient::new(rc.clone())),
            config_routes_api: Box::new(crate::apis::ConfigRoutesApiClient::new(rc.clone())),
            contract_routes_api: Box::new(crate::apis::ContractRoutesApiClient::new(rc.clone())),
            diagnostic_routes_api: Box::new(crate::apis::DiagnosticRoutesApiClient::new(rc.clone())),
            metadata_routes_api: Box::new(crate::apis::MetadataRoutesApiClient::new(rc.clone())),
            mosaic_routes_api: Box::new(crate::apis::MosaicRoutesApiClient::new(rc.clone())),
            namespace_routes_api: Box::new(crate::apis::NamespaceRoutesApiClient::new(rc.clone())),
            network_routes_api: Box::new(crate::apis::NetworkRoutesApiClient::new(rc.clone())),
            node_routes_api: Box::new(crate::apis::NodeRoutesApiClient::new(rc.clone())),
            transaction_routes_api: Box::new(crate::apis::TransactionRoutesApiClient::new(rc.clone())),
            upgrade_routes_api: Box::new(crate::apis::UpgradeRoutesApiClient::new(rc.clone())),
        }
    }

    pub fn account_routes_api(&self) -> &dyn crate::apis::AccountRoutesApi {
        self.account_routes_api.as_ref()
    }

    pub fn block_routes_api(&self) -> &dyn crate::apis::BlockRoutesApi {
        self.block_routes_api.as_ref()
    }

    pub fn chain_routes_api(&self) -> &dyn crate::apis::ChainRoutesApi {
        self.chain_routes_api.as_ref()
    }

    pub fn config_routes_api(&self) -> &dyn crate::apis::ConfigRoutesApi {
        self.config_routes_api.as_ref()
    }

    pub fn contract_routes_api(&self) -> &dyn crate::apis::ContractRoutesApi {
        self.contract_routes_api.as_ref()
    }

    pub fn diagnostic_routes_api(&self) -> &dyn crate::apis::DiagnosticRoutesApi {
        self.diagnostic_routes_api.as_ref()
    }

    pub fn metadata_routes_api(&self) -> &dyn crate::apis::MetadataRoutesApi {
        self.metadata_routes_api.as_ref()
    }

    pub fn mosaic_routes_api(&self) -> &dyn crate::apis::MosaicRoutesApi {
        self.mosaic_routes_api.as_ref()
    }

    pub fn namespace_routes_api(&self) -> &dyn crate::apis::NamespaceRoutesApi {
        self.namespace_routes_api.as_ref()
    }

    pub fn network_routes_api(&self) -> &dyn crate::apis::NetworkRoutesApi {
        self.network_routes_api.as_ref()
    }

    pub fn node_routes_api(&self) -> &dyn crate::apis::NodeRoutesApi {
        self.node_routes_api.as_ref()
    }

    pub fn transaction_routes_api(&self) -> &dyn crate::apis::TransactionRoutesApi {
        self.transaction_routes_api.as_ref()
    }

    pub fn upgrade_routes_api(&self) -> &dyn crate::apis::UpgradeRoutesApi {
        self.upgrade_routes_api.as_ref()
    }
}
