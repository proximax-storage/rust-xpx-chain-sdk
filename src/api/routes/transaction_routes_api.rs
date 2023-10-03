/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use hex::ToHex;
use serde_json::json;

use {
    ::std::fmt::{Debug, Display},
    hyper::Method,
};

use crate::{
    api::{error::Result, internally::valid_vec_len, TransactionStatusDto},
    errors_const::ERR_EMPTY_TRANSACTION_HASHES,
    transaction::{
        CosignatureSignedTransaction, SignedTransaction, Transaction, TransactionsStatus,
        TransactionStatus,
    },
    TransactionHash,
};
use crate::api::{
    __internal_get_transaction, __internal_get_transactions_by_group_with_pagination,
    TransactionQueryParams, value_to_transaction,
};
use crate::api::error::{Error, SiriusError};
use crate::api::request::ApiRequest;
use crate::api::routes::const_routes::{ANNOUNCE_TRANSACTION_ROUTE, TRANSACTIONS_BY_GROUP_ROUTE};
use crate::api::transport::service::Connection;
use crate::transaction::{TransactionGroupType, Transactions, TransactionSearch};

use super::{
    AGGREGATE_TRANSACTIONS_ROUTE, ANNOUNCE_AGGREGATE_COSIGNATURE_ROUTE, TRANSACTION_ROUTE,
    TRANSACTION_STATUS_ROUTE, TRANSACTIONS_ROUTE, TRANSACTIONS_STATUS_ROUTE,
};

/// Transaction ApiClient routes.
///
#[derive(Clone)]
pub struct TransactionRoutes(Connection);

/// Transaction related endpoints.
///
impl TransactionRoutes {
    pub(crate) fn new(client: Connection) -> Self {
        TransactionRoutes(client)
    }

    fn __client(&self) -> Connection {
        self.0.clone()
    }

    /// GetAnyTransaction returns Transaction for passed transaction id or hash.
    ///
    pub async fn get_any_transaction(
        &self,
        transaction_hash: TransactionHash,
    ) -> Result<Box<dyn Transaction>> {
        let txn_status = self.get_transaction_status(transaction_hash).await?;

        if txn_status.to_group_type() == TransactionGroupType::Failed {
            return Err(Error::SiriusError(SiriusError {
                code: "400".to_string(),
                message: txn_status.status,
            }));
        }

        self.get_transaction(transaction_hash, txn_status.to_group_type()).await
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
    ///use xpx_chain_sdk::api::ApiNode;
    /// use std::str::FromStr;
    /// use xpx_chain_sdk::TransactionHash;
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let hash: TransactionHash = TransactionHash::from_str("130171141CAE9D9ED6F62FD47CC316631986BBACD6B3D63930A9C46ED1ED764F").unwrap();
    /// let node_url = "http://api-2.testnet2.xpxsirius.io:3000";
    /// let client = ApiNode::from_static(node_url).connect().await.unwrap();
    ///
    /// let transaction_status = client.transaction_api().get_transaction_status( hash ).await;
    /// match transaction_status {
    ///     Ok(resp_info) => println!("{}", resp_info),
    ///     Err(err) => eprintln!("{:?}", err),
    /// }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an `TransactionStatus` for a given hash or
    /// whose error value is an `Error<VALUE>` describing the error that occurred.
    pub async fn get_transaction_status(
        &self,
        transaction_hash: TransactionHash,
    ) -> Result<TransactionStatus> {
        let mut req = ApiRequest::new(Method::GET, TRANSACTION_STATUS_ROUTE.to_string());

        req = req.with_path_param("hash".to_string(), transaction_hash.encode_hex::<String>());

        let dto: TransactionStatusDto = req.execute(self.__client()).await?;

        Ok(dto.compact()?)
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
    ///use ::std::str::FromStr;
    ///use xpx_chain_sdk::api::ApiNode;
    ///use xpx_chain_sdk::TransactionHash;
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///
    /// let hash_one: TransactionHash = TransactionHash::from_str("130171141CAE9D9ED6F62FD47CC316631986BBACD6B3D63930A9C46ED1ED764F").unwrap();
    /// let hash_two: TransactionHash = TransactionHash::from_str("5EC5C0E766B3DF81FBAD0E4FD794828002763905FEDC47208520E90FBED783B4").unwrap();
    ///
    /// let node_url = "http://api-2.testnet2.xpxsirius.io:3000";
    /// let client = ApiNode::from_static(node_url).connect().await.unwrap();
    ///
    /// let transactions_status = client.transaction_api().get_transactions_statuses( vec![hash_one,hash_two] ).await;
    ///
    /// match transactions_status {
    ///     Ok(statuses) => {
    ///         for status in statuses {
    ///             println!("{}", status)
    ///         }
    ///     }
    ///     Err(err) => eprintln!("{:?}", err),
    /// }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an vector of `TransactionStatus` for a
    /// given vector of transaction hashes or whose error value is an `Error<VALUE>` describing the
    /// error that occurred.
    pub async fn get_transactions_statuses(
        &self,
        hashes: Vec<TransactionHash>,
    ) -> Result<TransactionsStatus> {
        valid_vec_len(hashes.as_ref(), ERR_EMPTY_TRANSACTION_HASHES)?;

        let mut req = ApiRequest::new(Method::POST, TRANSACTIONS_STATUS_ROUTE.to_string());

        let body_param = json!({
			"hashes": hashes,
		});

        req = req.with_body_param(body_param);

        let dto: Vec<TransactionStatusDto> = req.execute(self.__client()).await?;

        let statuses: TransactionsStatus =
            dto.into_iter().map(move |status_dto| status_dto.compact().unwrap()).collect();

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
    /// use ::std::str::FromStr;
    /// use xpx_chain_sdk::api::ApiNode;
    /// use xpx_chain_sdk::transaction::TransactionGroupType;
    /// use xpx_chain_sdk::TransactionHash;
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let hash_one: TransactionHash = TransactionHash::from_str("130171141CAE9D9ED6F62FD47CC316631986BBACD6B3D63930A9C46ED1ED764F").unwrap();
    /// let node_url = "http://api-2.testnet2.xpxsirius.io:3000";
    /// let client = ApiNode::from_static(node_url).connect().await.unwrap();
    ///
    /// let transaction = client.transaction_api().get_transaction( hash_one , TransactionGroupType::Confirmed).await;
    ///
    /// match transaction {
    ///     Ok(resp_info) => println!("{}", resp_info),
    ///     Err(err) => eprintln!("{:?}", err),
    /// }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an `Box<dyn Transaction>` for given a
    /// transactionId or hash or whose error value is an `Error<VALUE>` describing the error that occurred.
    pub async fn get_transaction(
        &self,
        transaction_hash: TransactionHash,
        group: TransactionGroupType,
    ) -> Result<Box<dyn Transaction>> {
        let mut req = ApiRequest::new(Method::GET, TRANSACTION_ROUTE.to_string());

        req = req
            .with_path_param("transactionId".to_string(), transaction_hash.encode_hex::<String>())
            .with_path_param("group".to_string(), group.to_string());

        __internal_get_transaction(self.__client(), req).await
    }

    /// GetTransactionsByGroup returns an array of Transaction's for passed TransactionGroupType.
    pub async fn get_transactions_by_group(
        &self,
        group_type: TransactionGroupType,
        txn_query_params: Option<TransactionQueryParams>,
    ) -> Result<TransactionSearch> {
        __internal_get_transactions_by_group_with_pagination(
            self.__client(),
            TRANSACTIONS_BY_GROUP_ROUTE,
            group_type,
            txn_query_params,
        )
            .await
    }

    /// Get `Transactions` information.
    ///
    /// # Inputs
    ///
    /// * `transaction_ids` =    The vector of transaction ids.
    ///
    /// # Example
    ///
    /// ```
    ///
    ///use ::std::str::FromStr;
    ///use xpx_chain_sdk::api::ApiNode;
    ///use xpx_chain_sdk::TransactionHash;
    /// use xpx_chain_sdk::transaction::TransactionGroupType;
    ///#[tokio::main]
    ///async fn main() {
    ///
    /// let hash_one: TransactionHash = TransactionHash::from_str("130171141CAE9D9ED6F62FD47CC316631986BBACD6B3D63930A9C46ED1ED764F").unwrap();
    /// let hash_two: TransactionHash = TransactionHash::from_str("5EC5C0E766B3DF81FBAD0E4FD794828002763905FEDC47208520E90FBED783B4").unwrap();
    ///
    /// let node_url = "http://api-2.testnet2.xpxsirius.io:3000";
    /// let client = ApiNode::from_static(node_url).connect().await.unwrap();
    ///
    ///    let transactions_info = client.transaction_api().get_transactions( vec![hash_one,hash_two], TransactionGroupType::Confirmed ).await;
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
    /// Returns a Future `Result` whose okay value is an `Transactions` for a given vector of
    /// transactionIds or whose error value is an `Error<VALUE>` describing the error that occurred.
    pub async fn get_transactions(
        &self,
        hashes: Vec<TransactionHash>,
        group: TransactionGroupType,
    ) -> Result<Transactions> {
        let body_param = json!({
			"transactionIds": hashes,
		});

        let mut req = ApiRequest::new(Method::POST, TRANSACTIONS_ROUTE.to_string());

        req = req
            .with_body_param(body_param)
            .with_path_param("group".to_string(), group.to_string());

        let body_value = req.execute(self.__client()).await?;
        value_to_transaction(body_value)
    }

    /// Announces a transaction to the network.
    ///
    /// # Inputs
    ///
    /// * `transaction_signed` =  An [signed_transaction].
    ///
    /// # Example
    ///
    /// ```
    ///
    ///
    ///use xpx_chain_sdk::api::ApiNode;
    ///use xpx_chain_sdk::{
    ///    account::{Account, Address},
    ///    message::PlainMessage,
    ///    mosaic::Mosaic,
    ///    transaction::{TransferTransaction}
    ///};
    ///
    ///const PRIVATE_KEY: &str = "5D3E959EB0CD69CC1DB6E9C62CB81EC52747AB56FA740CF18AACB5003429AD2E";
    ///
    ///#[tokio::main]
    ///async fn main() {
    /// let node_url = "http://api-2.testnet2.xpxsirius.io:3000";
    /// let client = ApiNode::from_static(node_url).connect().await.unwrap();
    ///
    /// let chain = client.chain_api().get_block_by_height(1).await.unwrap();
    ///
    /// let network_type = chain.network_type;
    /// let generation_hash = chain.generation_hash;
    ///
    ///    let account = Account::from_hex_private_key(PRIVATE_KEY, network_type).unwrap();
    ///
    ///    let recipient = Address::from_raw("VC4A3Z6ALFGJPYAGDK2CNE2JAXOMQKILYBVNLQFS").unwrap();
    ///
    ///    let message = PlainMessage::create("Transfer From ProximaX Rust SDK");
    ///
    ///    let transfer_transaction = TransferTransaction::builder(network_type)
    ///         .recipient(recipient)
    ///         .mosaics(vec![Mosaic::xpx(1)])
    ///         .message(message)
    ///         .max_fee(1000)
    ///         .build();
    ///
    ///    if let Err(ref err) = transfer_transaction {
    ///        panic!("{}", err)
    ///    }
    ///
    ///    let sig_transaction = account.sign_transaction(
    ///        transfer_transaction.unwrap(), generation_hash);
    ///
    ///    let sig_tx = match &sig_transaction {
    ///        Ok(sig) => sig,
    ///        Err(err) => panic!("{}", err),
    ///    };
    ///
    ///    let response = client.transaction_api().announce(&sig_tx).await;
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
    /// Returns a Future `Result` whose okay value is an `AnnounceTransactionInfo` or whose error
    /// value is an `Error<VALUE>` describing the error that occurred.
    pub async fn announce(
        &self,
        transaction_signed: &SignedTransaction,
    ) -> Result<AnnounceTransactionInfo> {
        self.__announce_transaction(transaction_signed, ANNOUNCE_TRANSACTION_ROUTE)
            .await
    }

    pub async fn announce_aggregate_bonded(
        &self,
        signed_transaction: &SignedTransaction,
    ) -> Result<AnnounceTransactionInfo> {
        self.__announce_transaction(signed_transaction, AGGREGATE_TRANSACTIONS_ROUTE)
            .await
    }

    pub async fn announce_aggregate_bonded_cosignature(
        &self,
        cosignature: &CosignatureSignedTransaction,
    ) -> Result<AnnounceTransactionInfo> {
        self.__announce_transaction(cosignature, ANNOUNCE_AGGREGATE_COSIGNATURE_ROUTE)
            .await
    }

    async fn __announce_transaction<T>(&self, tx: T, route: &str) -> Result<AnnounceTransactionInfo>
        where
                for<'de> T: serde::Serialize,
    {
        let mut req = ApiRequest::new(Method::PUT, route.to_string());

        req = req.with_body_param(tx);

        req.execute(self.__client()).await
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
