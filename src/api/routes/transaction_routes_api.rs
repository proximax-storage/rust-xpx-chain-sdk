/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    ::std::{
        fmt::{Debug, Display},
        future::Future,
        sync::Arc,
    },
    reqwest::Method,
};

use crate::{
    api::{
        internally::{str_to_hash, valid_vec_hash, valid_vec_len},
        request as __internal_request,
        sirius_client::ApiClient,
        TransactionDto, TransactionStatusDto,
    },
    errors_const::{ERR_EMPTY_TRANSACTION_HASHES, ERR_EMPTY_TRANSACTION_IDS},
    models::Result,
    transaction::{
        CosignatureSignedTransaction, SignedTransaction, Transaction, TransactionHashes,
        TransactionIds, TransactionStatus, Transactions, TransactionsStatus,
    },
};

use super::{
    ANNOUNCE_AGGREGATE_COSIGNATURE_ROUTE, ANNOUNCE_AGGREGATE_ROUTE, TRANSACTIONS_ROUTE,
    TRANSACTIONS_STATUS_ROUTE, TRANSACTION_ROUTE, TRANSACTION_STATUS_ROUTE,
};

/// Transaction ApiClient routes.
///
#[derive(Clone)]
pub struct TransactionRoutes(Arc<ApiClient>);

/// Transaction related endpoints.
///
impl TransactionRoutes {
    pub(crate) fn new(client: Arc<ApiClient>) -> Self {
        TransactionRoutes(client)
    }

    fn __client(self) -> Arc<ApiClient> {
        self.0
    }

    /// Get transaction status
    ///
    /// # Inputs
    ///
    /// * `hash` =    The transaction hash.
    ///
    /// # Example
    ///
    /// ```
    ///
    ///use xpx_chain_sdk::api::SiriusClient;
    ///
    ///const HASH: &str = "130171141CAE9D9ED6F62FD47CC316631986BBACD6B3D63930A9C46ED1ED764F";
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];
    /// let client = SiriusClient::new(node_url);
    ///
    ///    let transaction_status = client.transaction.get_transaction_status( HASH ).await;
    ///
    ///    match transaction_status {
    ///        Ok(resp_info) => println!("{}", resp_info),
    ///        Err(err) => eprintln!("{:?}", err),
    ///    }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an [TransactionStatus] for a given hash or
    /// whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn get_transaction_status(self, hash: &str) -> Result<TransactionStatus> {
        let mut req =
            __internal_request::Request::new(Method::GET, TRANSACTION_STATUS_ROUTE.to_string());

        req = req.with_path_param("hash".to_string(), str_to_hash(hash)?);

        let dto: Result<TransactionStatusDto> = req.execute(self.__client()).await;

        Ok(dto?.compact())
    }

    /// Get transactions status.
    ///
    /// # Inputs
    ///
    /// * `hashes` =    The vector of transaction hashes.
    ///
    /// # Example
    ///
    /// ```
    ///
    ///use xpx_chain_sdk::api::SiriusClient;
    ///
    ///const HASH_A: &str = "130171141CAE9D9ED6F62FD47CC316631986BBACD6B3D63930A9C46ED1ED764F";
    ///const HASH_B: &str = "5EC5C0E766B3DF81FBAD0E4FD794828002763905FEDC47208520E90FBED783B4";
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];
    ///use xpx_chain_sdk::api::SiriusClient;
    ///
    ///    let transactions_status = client.transaction.get_transactions_statuses( vec![HASH_A,HASH_B] ).await;
    ///
    ///    match transactions_status {
    ///        Ok(statuses) => {
    ///            for status in statuses {
    ///                println!("{}", status)
    ///            }
    ///        }
    ///        Err(err) => eprintln!("{:?}", err),
    ///    }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an vector of [TransactionStatus] for a
    /// given vector of transaction hashes or whose error value is an `Error<Value>` describing the
    /// error that occurred.
    pub async fn get_transactions_statuses(self, hashes: Vec<&str>) -> Result<TransactionsStatus> {
        valid_vec_len(&hashes, ERR_EMPTY_TRANSACTION_HASHES)?;

        valid_vec_hash(&hashes)?;

        let transaction_hashes = TransactionHashes::from(hashes);

        let mut req = __internal_request::Request::new(
            reqwest::Method::POST,
            TRANSACTIONS_STATUS_ROUTE.to_string(),
        );

        req = req.with_body_param(transaction_hashes);

        let dto: Vec<TransactionStatusDto> = req.execute(self.__client()).await?;

        let statuses: TransactionsStatus = dto
            .into_iter()
            .map(move |status_dto| status_dto.compact())
            .collect();

        Ok(statuses)
    }

    /// Get transaction information.
    ///
    /// # Inputs
    ///
    /// * `transaction_id` =    The transaction id or hash.
    ///
    /// # Example
    ///
    /// ```
    ///
    ///use xpx_chain_sdk::api::SiriusClient;
    ///
    ///const HASH: &str = "130171141CAE9D9ED6F62FD47CC316631986BBACD6B3D63930A9C46ED1ED764F";
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];
    /// let client = SiriusClient::new(node_url);
    ///
    ///    let transaction = client.transaction.get_transaction( HASH ).await;
    ///
    ///    match transaction {
    ///        Ok(resp_info) => println!("{}", resp_info),
    ///        Err(err) => eprintln!("{:?}", err),
    ///    }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an [Box<dyn Transaction>] for given a
    /// transactionId or hash or whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn get_transaction(self, transaction_id: &str) -> Result<Box<dyn Transaction>> {
        let mut req = __internal_request::Request::new(Method::GET, TRANSACTION_ROUTE.to_string());

        let mut id = transaction_id.to_string();
        if transaction_id.len() != 24 {
            id = str_to_hash(&transaction_id)?
        }

        req = req
            .with_path_param("transactionId".to_string(), id)
            .set_transaction();

        let version: Box<dyn TransactionDto> = req.execute(self.__client()).await?;

        Ok(version.compact()?)
    }

    /// Get [Transactions] information.
    ///
    /// # Inputs
    ///
    /// * `transaction_ids` =    The vector of transaction ids.
    ///
    /// # Example
    ///
    /// ```
    ///
    ///use xpx_chain_sdk::api::SiriusClient;
    ///
    ///const HASH_A: &str = "130171141CAE9D9ED6F62FD47CC316631986BBACD6B3D63930A9C46ED1ED764F";
    ///const HASH_B: &str = "5EC5C0E766B3DF81FBAD0E4FD794828002763905FEDC47208520E90FBED783B4";
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];
    /// let client = SiriusClient::new(node_url);
    ///
    ///    let transactions_info = client.transaction.get_transactions( vec![HASH_A,HASH_B] ).await;
    ///
    ///    match transactions_info {
    ///        Ok(transactions) => {
    ///            for transaction in transactions {
    ///                println!("{}", transaction)
    ///            }
    ///        }
    ///        Err(err) => eprintln!("{:?}", err),
    ///    }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an [Transactions] for a given vector of
    /// transactionIds or whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn get_transactions(self, transaction_ids: Vec<&str>) -> Result<Transactions> {
        valid_vec_len(&transaction_ids, ERR_EMPTY_TRANSACTION_IDS)?;

        valid_vec_hash(&transaction_ids)?;

        let ids = TransactionIds::from(transaction_ids);

        let mut req =
            __internal_request::Request::new(Method::POST, TRANSACTIONS_ROUTE.to_string());

        req = req.with_body_param(ids).set_transaction_vec();

        let dto: Vec<Box<dyn TransactionDto>> = req.execute(self.__client()).await?;

        let mut transactions_info: Transactions = vec![];
        for transaction_dto in dto.into_iter() {
            transactions_info.push(transaction_dto.compact()?);
        }

        Ok(transactions_info)
    }

    /// Announces a transaction to the network.
    ///
    /// # Inputs
    ///
    /// * `transaction_signed` =    An [signed_transaction].
    ///
    /// # Example
    ///
    /// ```
    ///
    ///
    ///use xpx_chain_sdk::api::SiriusClient;
    ///use xpx_chain_sdk::{
    ///    account::{Account, Address},
    ///    message::PlainMessage,
    ///    mosaic::Mosaic,
    ///    network::PUBLIC_TEST,
    ///    transaction::{Deadline, TransferTransaction}
    ///};
    ///
    ///const PRIVATE_KEY: &str = "5D3E959EB0CD69CC1DB6E9C62CB81EC52747AB56FA740CF18AACB5003429AD2E";
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];
    /// let client = SiriusClient::new(node_url);
    ///
    ///    let generation_hash = client.generation_hash().await;
    ///
    ///    // let network_type = client.network_type().await;
    ///    let network_type = PUBLIC_TEST;
    ///
    ///    // Deadline default 1 hour
    ///    // let deadline = Deadline::new(1, 0, 0);
    ///    let deadline = Deadline::default();
    ///
    ///    let account = Account::from_private_key(PRIVATE_KEY, network_type).unwrap();
    ///
    ///    let recipient = Address::from_raw("VC4A3Z6ALFGJPYAGDK2CNE2JAXOMQKILYBVNLQFS").unwrap();
    ///
    ///    let message = PlainMessage::new("Transfer From ProximaX Rust SDK");
    ///
    ///    let transfer_transaction = TransferTransaction::new(
    ///        deadline,
    ///        recipient,
    ///        vec![Mosaic::xpx(1)],
    ///        message,
    ///        network_type,
    ///    );
    ///
    ///    if let Err(err) = &transfer_transaction {
    ///        panic!("{}", err)
    ///    }
    ///
    ///    let sig_transaction = account.sign(
    ///        &transfer_transaction.unwrap(), &generation_hash);
    ///
    ///    let sig_tx = match &sig_transaction {
    ///        Ok(sig) => sig,
    ///        Err(err) => panic!("{}", err),
    ///    };
    ///
    ///    let response = client.transaction.announce(&sig_tx).await;
    ///
    ///    match response {
    ///        Ok(resp) => println!("{}", resp),
    ///        Err(err) => eprintln!("{:?}", err),
    ///    }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an [AnnounceTransactionInfo] or whose error
    /// value is an `Error<Value>` describing the error that occurred.
    pub async fn announce(
        self,
        transaction_signed: &SignedTransaction,
    ) -> Result<AnnounceTransactionInfo> {
        self.__announce_transaction(transaction_signed, TRANSACTIONS_ROUTE)
            .await
    }

    pub async fn announce_aggregate_bonded(
        self,
        signed_transaction: &SignedTransaction,
    ) -> Result<AnnounceTransactionInfo> {
        self.__announce_transaction(signed_transaction, ANNOUNCE_AGGREGATE_ROUTE)
            .await
    }

    pub async fn announce_aggregate_bonded_cosignature(
        self,
        cosignature: &CosignatureSignedTransaction,
    ) -> Result<AnnounceTransactionInfo> {
        self.__announce_transaction(cosignature, ANNOUNCE_AGGREGATE_COSIGNATURE_ROUTE)
            .await
    }

    fn __announce_transaction<T>(
        self,
        tx: T,
        route: &str,
    ) -> impl Future<Output = Result<AnnounceTransactionInfo>>
    where
        for<'de> T: serde::Serialize,
    {
        let mut req = __internal_request::Request::new(Method::PUT, route.to_string());

        req = req.with_body_param(tx);

        async { req.execute(self.__client()).await }
    }
}

#[derive(Debug, Deserialize)]
pub struct AnnounceTransactionInfo {
    pub message: String,
}

impl Display for AnnounceTransactionInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", &self.message)
    }
}
