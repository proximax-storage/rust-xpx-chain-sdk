/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use serde_json::json;
use {
    ::std::{collections::HashMap, sync::Arc},
    reqwest::Method,
};

use crate::api::routes::const_routes::{
    AGGREGATE_TRANSACTIONS_ROUTE, TRANSACTIONS_ROUTE, TRANSACTIONS_UNCONFIRMED_ROUTE,
};
use crate::api::{QueryParam, TransactionQueryParams, __internal_get_transactions};
use crate::transaction::{AggregateTransaction, TransactionSearch};
use crate::{
    account::{AccountInfo, AccountName, AccountProperties, AccountsId, Address, PublicAccount},
    api::{
        error::Result,
        internally::{str_to_account_id, valid_vec_len},
        request as __internal_request, AccountInfoDto, AccountNamesDto, AccountPropertiesInfoDto,
        ApiClient, MultisigAccountGraphInfoDto, MultisigAccountInfoDto,
    },
    errors_const::ERR_EMPTY_ADDRESSES_IDS,
    multisig::{MultisigAccountGraphInfo, MultisigAccountInfo},
};

use super::{
    ACCOUNTS_PROPERTIES_ROUTE, ACCOUNTS_ROUTE, ACCOUNT_NAMES_ROUTE, ACCOUNT_PROPERTIES_ROUTE,
    ACCOUNT_ROUTE, MULTISIG_ACCOUNT_GRAPH_INFO_ROUTE, MULTISIG_ACCOUNT_ROUTE,
};

/// Account ApiClient routes.
///
#[derive(Clone)]
pub struct AccountRoutes(Arc<ApiClient>);

/// Account related endpoints.
///
impl AccountRoutes {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        AccountRoutes(client)
    }

    fn __client(&self) -> Arc<ApiClient> {
        Arc::clone(&self.0)
    }

    /// Get `Account` information
    ///
    /// # Inputs
    ///
    /// * `account_id` =    The public key or address of the account.
    ///
    /// # Example
    ///
    /// ```
    ///
    ///use xpx_chain_sdk::api::SiriusClient;
    ///
    ///const PUBLIC_KEY: &str = "93C3B9075649F59BD88573ADC55B8915B12390A47C76F0C45F362ED0800BE237";
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = vec!["https://api-2.testnet2.xpxsirius.io"];
    /// let client = SiriusClient::new_from_urls(&node_url).await.unwrap();
    /// let account_info = client.account_api().account_info( PUBLIC_KEY).await;
    ///
    /// match account_info {
    ///        Ok(resp_info) => println!("{}", resp_info),
    ///        Err(err) => eprintln!("{:?}", err),
    /// }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an `AccountInfo` the account information or
    /// whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn account_info(&self, account_id: &str) -> Result<AccountInfo> {
        let id = str_to_account_id(account_id)?;

        let mut req = __internal_request::Request::new(Method::GET, ACCOUNT_ROUTE.to_string());

        req = req.with_path_param("address_id".to_string(), id);

        let dto: Result<AccountInfoDto> = req.execute(self.__client()).await;

        Ok(dto?.compact()?)
    }

    /// Get `Accounts` information
    ///
    /// # Inputs
    ///
    /// * `accounts_id` =    The array of public keys String or The array of addresses String.
    ///
    /// # Example
    ///
    /// ```
    ///
    ///use xpx_chain_sdk::api::SiriusClient;
    ///
    ///const PUBLIC_KEY_A: &str = "93C3B9075649F59BD88573ADC55B8915B12390A47C76F0C45F362ED0800BE237";
    ///const PUBLIC_KEY_B: &str = "3B49BF0A08BB7528E54BB803BEEE0D935B2C800364917B6EFF331368A4232FD5";
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = vec!["https://api-2.testnet2.xpxsirius.io"];
    /// let client = SiriusClient::new_from_urls(&node_url).await.unwrap();
    /// let accounts_info = client.account_api().accounts_info( vec![PUBLIC_KEY_A, PUBLIC_KEY_B]).await;
    ///
    ///    match accounts_info {
    ///        Ok(tx) => {
    ///            for info in tx {
    ///                println!("{}", info)
    ///            }
    ///        }
    ///        Err(err) => eprintln!("{:?}", err)
    ///    }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an vector of `AccountInfo` or
    /// whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn accounts_info(&self, accounts_id: Vec<&str>) -> Result<Vec<AccountInfo>> {
        valid_vec_len(&accounts_id, ERR_EMPTY_ADDRESSES_IDS)?;

        let accounts = AccountsId::from(accounts_id);

        let mut req = __internal_request::Request::new(Method::POST, ACCOUNTS_ROUTE.to_string());

        req = req.with_body_param(&accounts);

        let dto: Vec<AccountInfoDto> = req.execute(self.__client()).await?;

        let mut accounts_info: Vec<AccountInfo> = vec![];
        for account_dto in dto.into_iter() {
            accounts_info.push(account_dto.compact()?);
        }

        Ok(accounts_info)
    }

    /// Get readable names for a set of accounts_id.
    ///
    /// # Inputs
    ///
    /// * `accounts_id` =    The array of public keys String or The array of addresses String.
    ///
    /// # Example
    ///
    /// ```
    ///
    ///use xpx_chain_sdk::api::SiriusClient;
    ///
    ///const PUBLIC_KEY_A: &str = "93C3B9075649F59BD88573ADC55B8915B12390A47C76F0C45F362ED0800BE237";
    ///const PUBLIC_KEY_B: &str = "3B49BF0A08BB7528E54BB803BEEE0D935B2C800364917B6EFF331368A4232FD5";
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = vec!["https://api-2.testnet2.xpxsirius.io"];
    /// let client = SiriusClient::new_from_urls(&node_url).await.unwrap();
    /// let accounts_names = client.account_api().accounts_names( vec![PUBLIC_KEY_A, PUBLIC_KEY_B]).await;
    ///
    ///    match accounts_names {
    ///        Ok(names) => {
    ///            for name in names {
    ///                println!("{}", name)
    ///            }
    ///        }
    ///        Err(err) => eprintln!("{:?}", err)
    ///    }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an vector of `AccountName` or
    /// whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn accounts_names(self, accounts_id: Vec<&str>) -> Result<Vec<AccountName>> {
        valid_vec_len(&accounts_id, ERR_EMPTY_ADDRESSES_IDS)?;

        let accounts = AccountsId::from(accounts_id);

        let mut req =
            __internal_request::Request::new(Method::POST, ACCOUNT_NAMES_ROUTE.to_string());

        req = req.with_body_param(&accounts);

        let dto: Vec<AccountNamesDto> = req.execute(self.__client()).await?;

        let mut accounts_names: Vec<AccountName> = vec![];
        for accounts_dto in dto.into_iter() {
            accounts_names.push(accounts_dto.compact()?);
        }

        Ok(accounts_names)
    }

    /// Get multisig `Account` information
    ///
    /// # Inputs
    ///
    /// * `account_id` =    The public key or address of the account.
    ///
    /// # Example
    ///
    /// ```
    ///
    ///use xpx_chain_sdk::api::SiriusClient;
    ///
    ///const PUBLIC_KEY: &str = "93C3B9075649F59BD88573ADC55B8915B12390A47C76F0C45F362ED0800BE237";
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = vec!["https://api-2.testnet2.xpxsirius.io"];
    /// let client = SiriusClient::new_from_urls(&node_url).await.unwrap();
    /// let account_multisig = client.account_api().account_multisig( PUBLIC_KEY).await;
    ///
    /// match account_multisig {
    ///        Ok(resp_info) => println!("{}", resp_info),
    ///        Err(err) => eprintln!("{:?}", err),
    /// }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an `MultisigAccountInfo` the account information or
    /// whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn account_multisig(&self, account_id: &str) -> Result<MultisigAccountInfo> {
        let id = str_to_account_id(account_id)?;

        let mut req =
            __internal_request::Request::new(Method::GET, MULTISIG_ACCOUNT_ROUTE.to_string());

        req = req.with_path_param("address_id".to_string(), id);

        let dto: Result<MultisigAccountInfoDto> = req.execute(self.__client()).await;

        Ok(dto?.compact()?)
    }

    /// Get multisig `Account` graph information
    ///
    /// # Inputs
    ///
    /// * `account_id` =    The public key or address of the account.
    ///
    /// # Example
    ///
    /// ```
    ///
    ///use xpx_chain_sdk::api::SiriusClient;
    ///
    ///const PUBLIC_KEY: &str = "93C3B9075649F59BD88573ADC55B8915B12390A47C76F0C45F362ED0800BE237";
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = vec!["https://api-2.testnet2.xpxsirius.io"];
    /// let client = SiriusClient::new_from_urls(&node_url).await.unwrap();
    /// let account_multisig_graph = client.account_api().account_multisig_graph( PUBLIC_KEY).await;
    ///
    /// match account_multisig_graph {
    ///        Ok(resp_info) => println!("{}", resp_info),
    ///        Err(err) => eprintln!("{:?}", err),
    /// }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an `MultisigAccountGraphInfo` the account information or
    /// whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn account_multisig_graph(
        &self,
        account_id: &str,
    ) -> Result<MultisigAccountGraphInfo> {
        let id = str_to_account_id(account_id)?;

        let mut req = __internal_request::Request::new(
            Method::GET,
            MULTISIG_ACCOUNT_GRAPH_INFO_ROUTE.to_string(),
        );

        req = req.with_path_param("address_id".to_string(), id);

        let dto: Result<Vec<MultisigAccountGraphInfoDto>> = req.execute(self.__client()).await;

        let mut multisig_accounts: HashMap<i16, Vec<MultisigAccountInfo>> = HashMap::new();
        for graph_info_dto in dto?.into_iter() {
            let info = graph_info_dto.compact()?;
            multisig_accounts.insert(graph_info_dto.level, info);
        }

        Ok(MultisigAccountGraphInfo { multisig_accounts })
    }

    /// Get `Account` configurable properties information.
    ///
    /// # Inputs
    ///
    /// * `address` =    The `Address` of the account.
    ///
    /// # Example
    ///
    /// ```
    ///
    ///use xpx_chain_sdk::api::SiriusClient;
    ///use xpx_chain_sdk::account::Address;
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///
    /// let node_url = vec!["https://api-2.testnet2.xpxsirius.io"];
    /// let client = SiriusClient::new_from_urls(&node_url).await.unwrap();
    ///
    /// let address = Address::from_raw("VC4A3Z6ALFGJPYAGDK2CNE2JAXOMQKILYBVNLQFS").unwrap();
    /// let account_properties = client.account_api().account_properties( address).await;
    ///
    /// match account_properties {
    ///        Ok(resp_info) => println!("{}", resp_info),
    ///        Err(err) => eprintln!("{:?}", err),
    /// }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an `AccountProperties` the account information or
    /// whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn account_properties(&self, address: Address) -> Result<AccountProperties> {
        let mut req =
            __internal_request::Request::new(Method::GET, ACCOUNT_PROPERTIES_ROUTE.to_string());

        req = req.with_path_param("address_id".to_string(), address.address_str());

        let dto: Result<AccountPropertiesInfoDto> = req.execute(self.__client()).await;

        dto?.compact()
    }

    /// Get account properties for given vec of addresses
    ///
    /// # Inputs
    ///
    /// * `addresses` =    The vec of `Address`.
    ///
    /// # Example
    ///
    /// ```
    ///
    ///use xpx_chain_sdk::api::SiriusClient;
    ///use xpx_chain_sdk::account::Address;
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///
    /// let node_url = vec!["https://api-2.testnet2.xpxsirius.io"];
    /// let client = SiriusClient::new_from_urls(&node_url).await.unwrap();
    ///
    /// let address_a = Address::from_raw("VCWN77662VNLFGQBQ6RIFTC7LFWO5BSA3U622CSK").unwrap();
    /// let address_b = Address::from_raw("VBWH2BUPITAIHNIPOQ322OB5Q5FF6GFPDR5SYMAA").unwrap();
    ///
    /// let accounts_properties = client.account_api().accounts_properties( vec![address_a, address_b]).await;
    ///
    ///    match accounts_properties {
    ///        Ok(properties) => {
    ///            for info in properties {
    ///                println!("{}", info)
    ///            }
    ///        }
    ///        Err(err) => eprintln!("{:?}", err)
    ///    }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an vector of `AccountProperties` or
    /// whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn accounts_properties(
        &self,
        addresses: Vec<Address>,
    ) -> Result<Vec<AccountProperties>> {
        valid_vec_len(&addresses, ERR_EMPTY_ADDRESSES_IDS)?;

        let accounts = AccountsId::from(
            addresses
                .into_iter()
                .map(|item| item.address_str())
                .collect::<Vec<_>>(),
        );

        let mut req =
            __internal_request::Request::new(Method::POST, ACCOUNTS_PROPERTIES_ROUTE.to_string());

        req = req.with_body_param(&accounts);

        let dto: Result<Vec<AccountPropertiesInfoDto>> = req.execute(self.__client()).await;

        let mut accounts_properties: Vec<AccountProperties> = vec![];
        for properties_dto in dto?.into_iter() {
            accounts_properties.push(properties_dto.compact()?);
        }

        Ok(accounts_properties)
    }

    /// Get Confirmed transactions
    ///
    /// Gets an List of transactions for which an account is the sender or receiver.
    pub async fn transactions(
        &self,
        public_account: PublicAccount,
        txn_query_params: Option<TransactionQueryParams>,
    ) -> Result<TransactionSearch> {
        let mut query_params_vec: Vec<QueryParam> = vec![];

        if let Some(mut query_params) = txn_query_params {
            query_params.public_key = Some(public_account.public_key_to_hex());
            query_params_vec.append(&mut query_params.to_query_params());
        } else {
            query_params_vec.push(QueryParam {
                name: "publicKey".to_string(),
                value: json!(public_account.public_key_to_hex()),
            });
        }

        __internal_get_transactions(self.__client(), TRANSACTIONS_ROUTE, query_params_vec).await
    }

    /// Get incoming transactions
    ///
    /// Gets an List of incoming transactions.
    /// A transaction is said to be incoming with respect to an
    /// account if the account is the recipient of the transaction.
    pub async fn incoming_transactions(
        &self,
        public_account: PublicAccount,
        txn_query_params: Option<TransactionQueryParams>,
    ) -> Result<TransactionSearch> {
        let mut query_params_vec: Vec<QueryParam> = vec![];

        if let Some(mut query_params) = txn_query_params {
            query_params.recipient_address = Some(public_account.address_str());
            query_params_vec.append(&mut query_params.to_query_params());
        } else {
            query_params_vec.push(QueryParam {
                name: "recipientAddress".to_string(),
                value: json!(public_account.address_str()),
            });
        }

        __internal_get_transactions(self.__client(), TRANSACTIONS_ROUTE, query_params_vec).await
    }

    /// Get outgoing transactions
    ///
    /// Gets an List of outgoing transactions.
    /// A transaction is said to be outgoing with respect to an
    /// account if the account is the sender of the transaction.
    pub async fn outgoing_transactions(
        &self,
        public_account: PublicAccount,
        txn_query_params: Option<TransactionQueryParams>,
    ) -> Result<TransactionSearch> {
        let mut query_params_vec: Vec<QueryParam> = vec![];

        if let Some(mut query_params) = txn_query_params {
            query_params.signer_public_key = Some(public_account.public_key_to_hex());
            query_params_vec.append(&mut query_params.to_query_params());
        } else {
            query_params_vec.push(QueryParam {
                name: "signerPublicKey".to_string(),
                value: json!(public_account.public_key_to_hex()),
            });
        }

        __internal_get_transactions(self.__client(), TRANSACTIONS_ROUTE, query_params_vec).await
    }

    /// Get Unconfirmed transactions
    ///
    /// Gets the List of transactions not included in a block where an account
    /// is the sender or receiver.
    pub async fn unconfirmed_transactions(
        &self,
        public_account: PublicAccount,
        txn_query_params: Option<TransactionQueryParams>,
    ) -> Result<TransactionSearch> {
        let mut query_params_vec: Vec<QueryParam> = vec![];

        if let Some(mut query_params) = txn_query_params {
            query_params.signer_public_key = Some(public_account.public_key_to_hex());
            query_params_vec.append(&mut query_params.to_query_params());
        } else {
            query_params_vec.push(QueryParam {
                name: "signerPublicKey".to_string(),
                value: json!(public_account.public_key_to_hex()),
            });
        }

        __internal_get_transactions(
            self.__client(),
            TRANSACTIONS_UNCONFIRMED_ROUTE,
            query_params_vec,
        )
        .await
    }

    /// Get aggregate bonded transactions information
    ///
    /// Gets an List of [aggregate bonded transactions] where the account is
    /// the sender or requires to cosign the transaction.
    pub async fn partial_transactions(
        &self,
        public_account: PublicAccount,
        txn_query_params: Option<TransactionQueryParams>,
    ) -> Result<Vec<AggregateTransaction>> {
        let mut query_params_vec: Vec<QueryParam> = vec![];

        if let Some(mut query_params) = txn_query_params {
            query_params.signer_public_key = Some(public_account.public_key_to_hex());
            query_params_vec.append(&mut query_params.to_query_params());
        } else {
            query_params_vec.push(QueryParam {
                name: "signerPublicKey".to_string(),
                value: json!(public_account.public_key_to_hex()),
            });
        }

        __internal_get_transactions(
            self.__client(),
            AGGREGATE_TRANSACTIONS_ROUTE,
            query_params_vec,
        )
        .await
        .map(|txns| {
            txns.transactions
                .into_iter()
                .map(|item| *item.downcast::<AggregateTransaction>())
                .collect::<Vec<_>>()
        })
    }
}
