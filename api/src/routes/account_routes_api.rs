// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use ::std::{collections::HashMap, future::Future, sync::Arc};

use reqwest::Method;

use sdk::{
    account::{AccountInfo, AccountName, AccountsId, PublicAccount},
    errors_const::ERR_EMPTY_ADDRESSES_IDS,
    multisig::{MultisigAccountGraphInfo, MultisigAccountInfo},
    transaction::Transactions,
};

use crate::{
    dtos::{
        AccountInfoDto, AccountNamesDto, MultisigAccountGraphInfoDto, MultisigAccountInfoDto,
        TransactionDto,
    },
    internally::{str_to_account_id, valid_vec_len, AccountTransactionsOption},
    request as __internal_request,
    sirius_client::ApiClient,
    Result,
};

use super::{
    ACCOUNTS_PROPERTIES_ROUTE, ACCOUNTS_ROUTE, ACCOUNT_NAMES_ROUTE, ACCOUNT_PROPERTIES_ROUTE,
    ACCOUNT_ROUTE, AGGREGATE_TRANSACTIONS_ROUTE, INCOMING_TRANSACTIONS_ROUTE,
    MULTISIG_ACCOUNT_GRAPH_INFO_ROUTE, MULTISIG_ACCOUNT_ROUTE, OUTGOING_TRANSACTIONS_ROUTE,
    TRANSACTIONS_BY_ACCOUNT_ROUTE, UNCONFIRMED_TRANSACTIONS_ROUTE,
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

    fn __client(self) -> Arc<ApiClient> {
        self.0
    }

    /// Get [Account] information
    ///
    /// # Inputs
    ///
    /// * `account_id` =    The public key or address of the account.
    ///
    /// # Example
    ///
    /// ```
    ///
    ///use xpx_chain_api::SiriusClient;
    ///
    ///const PUBLIC_KEY: &str = "93C3B9075649F59BD88573ADC55B8915B12390A47C76F0C45F362ED0800BE237";
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];
    /// let client = SiriusClient::new(node_url);
    ///
    ///    let account_info = client.account.account_info( PUBLIC_KEY).await;
    ///
    ///    match account_info {
    ///        Ok(resp_info) => println!("{}", resp_info),
    ///        Err(err) => eprintln!("{:?}", err),
    ///    }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an [AccountInfo] the account information or
    /// whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn account_info(self, account_id: &str) -> Result<AccountInfo> {
        let id = str_to_account_id(account_id)?;

        let mut req = __internal_request::Request::new(Method::GET, ACCOUNT_ROUTE.to_string());

        req = req.with_path_param("accountId".to_string(), id);

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
    ///use xpx_chain_api::SiriusClient;
    ///
    ///const PUBLIC_KEY_A: &str = "93C3B9075649F59BD88573ADC55B8915B12390A47C76F0C45F362ED0800BE237";
    ///const PUBLIC_KEY_B: &str = "3B49BF0A08BB7528E54BB803BEEE0D935B2C800364917B6EFF331368A4232FD5";
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];
    /// let client = SiriusClient::new(node_url);
    ///
    ///    let accounts_info = client.account.get_accounts_info( vec![PUBLIC_KEY_A, PUBLIC_KEY_B]).await;
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
    /// Returns a Future `Result` whose okay value is an vector of [AccountInfo] or
    /// whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn accounts_info(self, accounts_id: Vec<&str>) -> Result<Vec<AccountInfo>> {
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

    pub async fn account_multisig(self, account_id: &str) -> Result<MultisigAccountInfo> {
        let id = str_to_account_id(account_id)?;

        let mut req =
            __internal_request::Request::new(Method::GET, MULTISIG_ACCOUNT_ROUTE.to_string());

        req = req.with_path_param("accountId".to_string(), id);

        let dto: Result<MultisigAccountInfoDto> = req.execute(self.__client()).await;

        Ok(dto?.compact()?)
    }

    pub async fn account_multisig_graph(
        self,
        account_id: &str,
    ) -> Result<MultisigAccountGraphInfo> {
        let id = str_to_account_id(account_id)?;

        let mut req = __internal_request::Request::new(
            Method::GET,
            MULTISIG_ACCOUNT_GRAPH_INFO_ROUTE.to_string(),
        );

        req = req.with_path_param("accountId".to_string(), id);

        let dto: Result<Vec<MultisigAccountGraphInfoDto>> = req.execute(self.__client()).await;

        let mut multisig_accounts: HashMap<i16, Vec<MultisigAccountInfo>> = HashMap::new();
        for graph_info_dto in dto?.into_iter() {
            let info = graph_info_dto.compact()?;
            multisig_accounts.insert(graph_info_dto.level, info);
        }

        Ok(MultisigAccountGraphInfo { multisig_accounts })
    }

    pub async fn account_properties(self, account_id: &str) -> Result<()> {
        let id = str_to_account_id(account_id)?;

        let mut req =
            __internal_request::Request::new(Method::GET, ACCOUNT_PROPERTIES_ROUTE.to_string());

        req.with_path_param("accountId".to_string(), id);
        unimplemented!()
    }

    pub async fn accounts_properties(self, accounts_id: Vec<&str>) -> Result<Vec<()>> {
        valid_vec_len(&accounts_id, ERR_EMPTY_ADDRESSES_IDS)?;

        let accounts = AccountsId::from(accounts_id);

        let mut req =
            __internal_request::Request::new(Method::POST, ACCOUNTS_PROPERTIES_ROUTE.to_string());

        req.with_body_param(&accounts);

        unimplemented!()
    }

    pub async fn transactions(
        self,
        public_account: &PublicAccount,
        page_size: Option<i32>,
        id: Option<&str>,
        ordering: Option<&str>,
    ) -> Result<Transactions> {
        let transactions_options = AccountTransactionsOption::new(page_size, id, ordering)?;

        self.__internal_transactions(
            public_account,
            TRANSACTIONS_BY_ACCOUNT_ROUTE,
            transactions_options,
        )
        .await
    }

    pub async fn incoming_transactions(
        self,
        public_account: &PublicAccount,
        page_size: Option<i32>,
        id: Option<&str>,
        ordering: Option<&str>,
    ) -> Result<Transactions> {
        let transactions_options = AccountTransactionsOption::new(page_size, id, ordering)?;

        self.__internal_transactions(
            public_account,
            INCOMING_TRANSACTIONS_ROUTE,
            transactions_options,
        )
        .await
    }

    pub async fn outgoing_transactions(
        self,
        public_account: &PublicAccount,
        page_size: Option<i32>,
        id: Option<&str>,
        ordering: Option<&str>,
    ) -> Result<Transactions> {
        let transactions_options = AccountTransactionsOption::new(page_size, id, ordering)?;

        self.__internal_transactions(
            public_account,
            OUTGOING_TRANSACTIONS_ROUTE,
            transactions_options,
        )
        .await
    }

    pub async fn unconfirmed_transactions(
        self,
        public_account: &PublicAccount,
        page_size: Option<i32>,
        id: Option<&str>,
        ordering: Option<&str>,
    ) -> Result<Transactions> {
        let transactions_options = AccountTransactionsOption::new(page_size, id, ordering)?;

        self.__internal_transactions(
            public_account,
            UNCONFIRMED_TRANSACTIONS_ROUTE,
            transactions_options,
        )
        .await
    }

    pub async fn partial_transactions(
        self,
        public_account: &PublicAccount,
        page_size: Option<i32>,
        id: Option<&str>,
        ordering: Option<&str>,
    ) -> Result<Transactions> {
        let transactions_options = AccountTransactionsOption::new(page_size, id, ordering)?;

        self.__internal_transactions(
            public_account,
            AGGREGATE_TRANSACTIONS_ROUTE,
            transactions_options,
        )
        .await
    }

    fn __internal_transactions(
        self,
        public_account: &PublicAccount,
        route: &str,
        options: AccountTransactionsOption,
    ) -> impl Future<Output = Result<Transactions>> {
        let mut req = __internal_request::Request::new(Method::GET, route.to_string());

        if let Some(s) = options.page_size {
            req = req.with_query_param("pageSize".to_string(), s.to_string());
        }
        if let Some(s) = options.id {
            req = req.with_query_param("id".to_string(), s.to_string());
        }
        if let Some(s) = options.ordering {
            req = req.with_query_param("ordering".to_string(), s.to_string());
        }

        req = req.with_path_param(
            "publicKey".to_string(),
            public_account.public_key.to_string(),
        );

        req = req.set_transaction_vec();

        async {
            let dto: Vec<Box<dyn TransactionDto>> = req.execute(self.__client()).await?;

            let mut transactions_info: Transactions = vec![];
            for transaction_dto in dto.into_iter() {
                transactions_info.push(transaction_dto.compact()?)
            }

            Ok(transactions_info)
        }
    }
}
