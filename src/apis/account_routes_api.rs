use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct AccountRoutesApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> AccountRoutesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> AccountRoutesApiClient<C> {
        AccountRoutesApiClient {
            configuration,
        }
    }
}

pub trait AccountRoutesApi {
    fn get_account_info(&self, account_id: &str) -> Box<dyn Future<Item = crate::models::AccountInfoDto, Error = Error<serde_json::Value>>>;
    fn get_account_multisig(&self, account_id: &str) -> Box<dyn Future<Item = crate::models::MultisigAccountInfoDto, Error = Error<serde_json::Value>>>;
    fn get_account_multisig_graph(&self, account_id: &str) -> Box<dyn Future<Item = Vec<crate::models::MultisigAccountGraphInfoDto>, Error = Error<serde_json::Value>>>;
    fn get_account_properties(&self, account_id: &str) -> Box<dyn Future<Item = crate::models::AccountPropertiesInfoDto, Error = Error<serde_json::Value>>>;
    fn get_account_properties_from_accounts(&self, account_ids: crate::models::AccountIds) -> Box<dyn Future<Item = Vec<crate::models::AccountPropertiesInfoDto>, Error = Error<serde_json::Value>>>;
    fn get_accounts_info(&self, account_ids: crate::models::AccountIds) -> Box<dyn Future<Item = Vec<crate::models::AccountInfoDto>, Error = Error<serde_json::Value>>>;
    fn get_accounts_names(&self, account_ids: crate::models::AccountIds) -> Box<dyn Future<Item = Vec<crate::models::AccountNamesDto>, Error = Error<serde_json::Value>>>;
    fn incoming_transactions(&self, public_key: &str, page_size: Option<i32>, id: Option<&str>, ordering: Option<&str>) -> Box<dyn Future<Item = Vec<crate::models::TransactionInfoDto>, Error = Error<serde_json::Value>>>;
    fn outgoing_transactions(&self, public_key: &str, page_size: Option<i32>, id: Option<&str>, ordering: Option<&str>) -> Box<dyn Future<Item = Vec<crate::models::TransactionInfoDto>, Error = Error<serde_json::Value>>>;
    fn partial_transactions(&self, public_key: &str, page_size: Option<i32>, id: Option<&str>, ordering: Option<&str>) -> Box<dyn Future<Item = Vec<crate::models::TransactionInfoDto>, Error = Error<serde_json::Value>>>;
    fn transactions(&self, public_key: &str, page_size: Option<i32>, id: Option<&str>, ordering: Option<&str>) -> Box<dyn Future<Item = Vec<crate::models::TransactionInfoDto>, Error = Error<serde_json::Value>>>;
    fn unconfirmed_transactions(&self, public_key: &str, page_size: Option<i32>, id: Option<&str>, ordering: Option<&str>) -> Box<dyn Future<Item = Vec<crate::models::TransactionInfoDto>, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>AccountRoutesApi for AccountRoutesApiClient<C> {
    fn get_account_info(&self, account_id: &str) -> Box<dyn Future<Item = crate::models::AccountInfoDto, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/account/{accountId}".to_string())
        ;
        req = req.with_path_param("accountId".to_string(), account_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_account_multisig(&self, account_id: &str) -> Box<dyn Future<Item = crate::models::MultisigAccountInfoDto, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/account/{accountId}/multisig".to_string())
        ;
        req = req.with_path_param("accountId".to_string(), account_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_account_multisig_graph(&self, account_id: &str) -> Box<dyn Future<Item = Vec<crate::models::MultisigAccountGraphInfoDto>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/account/{accountId}/multisig/graph".to_string())
        ;
        req = req.with_path_param("accountId".to_string(), account_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_account_properties(&self, account_id: &str) -> Box<dyn Future<Item = crate::models::AccountPropertiesInfoDto, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/account/{accountId}/properties/".to_string())
        ;
        req = req.with_path_param("accountId".to_string(), account_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_account_properties_from_accounts(&self, account_ids: crate::models::AccountIds) -> Box<dyn Future<Item = Vec<crate::models::AccountPropertiesInfoDto>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/account/properties".to_string())
        ;
        req = req.with_body_param(account_ids);

        req.execute(self.configuration.borrow())
    }

    fn get_accounts_info(&self, account_ids: crate::models::AccountIds) -> Box<dyn Future<Item = Vec<crate::models::AccountInfoDto>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/account".to_string())
        ;
        req = req.with_body_param(account_ids);

        req.execute(self.configuration.borrow())
    }

    fn get_accounts_names(&self, account_ids: crate::models::AccountIds) -> Box<dyn Future<Item = Vec<crate::models::AccountNamesDto>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/account/names".to_string())
        ;
        req = req.with_body_param(account_ids);

        req.execute(self.configuration.borrow())
    }

    fn incoming_transactions(&self, public_key: &str, page_size: Option<i32>, id: Option<&str>, ordering: Option<&str>) -> Box<dyn Future<Item = Vec<crate::models::TransactionInfoDto>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/account/{publicKey}/transactions/incoming".to_string())
        ;
        if let Some(ref s) = page_size {
            req = req.with_query_param("pageSize".to_string(), s.to_string());
        }
        if let Some(ref s) = id {
            req = req.with_query_param("id".to_string(), s.to_string());
        }
        if let Some(ref s) = ordering {
            req = req.with_query_param("ordering".to_string(), s.to_string());
        }
        req = req.with_path_param("publicKey".to_string(), public_key.to_string());

        req.execute(self.configuration.borrow())
    }

    fn outgoing_transactions(&self, public_key: &str, page_size: Option<i32>, id: Option<&str>, ordering: Option<&str>) -> Box<dyn Future<Item = Vec<crate::models::TransactionInfoDto>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/account/{publicKey}/transactions/outgoing".to_string())
        ;
        if let Some(ref s) = page_size {
            req = req.with_query_param("pageSize".to_string(), s.to_string());
        }
        if let Some(ref s) = id {
            req = req.with_query_param("id".to_string(), s.to_string());
        }
        if let Some(ref s) = ordering {
            req = req.with_query_param("ordering".to_string(), s.to_string());
        }
        req = req.with_path_param("publicKey".to_string(), public_key.to_string());

        req.execute(self.configuration.borrow())
    }

    fn partial_transactions(&self, public_key: &str, page_size: Option<i32>, id: Option<&str>, ordering: Option<&str>) -> Box<dyn Future<Item = Vec<crate::models::TransactionInfoDto>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/account/{publicKey}/transactions/partial".to_string())
        ;
        if let Some(ref s) = page_size {
            req = req.with_query_param("pageSize".to_string(), s.to_string());
        }
        if let Some(ref s) = id {
            req = req.with_query_param("id".to_string(), s.to_string());
        }
        if let Some(ref s) = ordering {
            req = req.with_query_param("ordering".to_string(), s.to_string());
        }
        req = req.with_path_param("publicKey".to_string(), public_key.to_string());

        req.execute(self.configuration.borrow())
    }

    fn transactions(&self, public_key: &str, page_size: Option<i32>, id: Option<&str>, ordering: Option<&str>) -> Box<dyn Future<Item = Vec<crate::models::TransactionInfoDto>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/account/{publicKey}/transactions".to_string())
        ;
        if let Some(ref s) = page_size {
            req = req.with_query_param("pageSize".to_string(), s.to_string());
        }
        if let Some(ref s) = id {
            req = req.with_query_param("id".to_string(), s.to_string());
        }
        if let Some(ref s) = ordering {
            req = req.with_query_param("ordering".to_string(), s.to_string());
        }
        req = req.with_path_param("publicKey".to_string(), public_key.to_string());

        req.execute(self.configuration.borrow())
    }

    fn unconfirmed_transactions(&self, public_key: &str, page_size: Option<i32>, id: Option<&str>, ordering: Option<&str>) -> Box<dyn Future<Item = Vec<crate::models::TransactionInfoDto>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/account/{publicKey}/transactions/unconfirmed".to_string())
        ;
        if let Some(ref s) = page_size {
            req = req.with_query_param("pageSize".to_string(), s.to_string());
        }
        if let Some(ref s) = id {
            req = req.with_query_param("id".to_string(), s.to_string());
        }
        if let Some(ref s) = ordering {
            req = req.with_query_param("ordering".to_string(), s.to_string());
        }
        req = req.with_path_param("publicKey".to_string(), public_key.to_string());

        req.execute(self.configuration.borrow())
    }

}
