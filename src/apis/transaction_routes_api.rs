use ::std::{fmt::{Debug, Display}, sync::Arc};

use hyper::{client::connect::Connect, Method};

use crate::{
    models::{
        transaction::{
            SignedTransaction,
            Transaction,
            TransactionHashes,
            TransactionIds,
            TransactionStatus,
            TransactionStatusDto,
            TransactionDto
        },
        errors::{ERR_EMPTY_TRANSACTION_HASHES, ERR_EMPTY_TRANSACTION_IDS}
    }};

use super::{
    internally::{valid_hash, valid_vec_hash, valid_vec_len},
    request as __internal_request,
    Result,
    sirius_client::ApiClient
};
use crate::models::transaction::{Transactions, TransactionsStatus};

/// Transaction ApiClient routes.
///
#[derive(Clone)]
pub struct TransactionRoutes<C: Connect> (Arc<ApiClient<C>>);

const TRANSACTIONS_ROUTE: &str = "/transaction";
const TRANSACTION_ROUTE: &str = "/transaction/{transactionId}";
const TRANSACTION_STATUS_ROUTE: &str = "/transaction/{hash}/status";
const TRANSACTIONS_STATUS_ROUTE: &str = "/transaction/statuses";
const ANNOUNCE_AGGREGATE_ROUTE: &str = "/transaction/partial";
const ANNOUNCE_AGGREGATE_COSIGNATURE_ROUTE: &str = "/transaction/cosignature";

/// Transaction related endpoints.
///
impl<C: Connect> TransactionRoutes<C> where
    C: Clone + Send + Sync + 'static,
{
    pub(crate) fn new(client: Arc<ApiClient<C>>) -> Self {
        TransactionRoutes(client)
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
    ///use hyper::Client;
    ///use xpx_chain_sdk::sirius_client::SiriusClient;
    ///
    ///const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";
    ///const HASH: &str = "130171141CAE9D9ED6F62FD47CC316631986BBACD6B3D63930A9C46ED1ED764F";
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///
    ///    let client = SiriusClient::new(NODE_URL, Client::new());
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
        valid_hash(hash)?;

        let mut req = __internal_request::Request::new(
            Method::GET,
            TRANSACTION_STATUS_ROUTE.to_string(),
        );

        req = req.with_path_param("hash".to_string(), hash.to_string());

        let dto: Result<TransactionStatusDto> = req.execute(self.0).await;

        Ok(dto?.to_struct())
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
    ///use hyper::Client;
    ///use xpx_chain_sdk::sirius_client::SiriusClient;
    ///
    ///const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";
    ///const HASH_A: &str = "130171141CAE9D9ED6F62FD47CC316631986BBACD6B3D63930A9C46ED1ED764F";
    ///const HASH_B: &str = "5EC5C0E766B3DF81FBAD0E4FD794828002763905FEDC47208520E90FBED783B4";
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///
    ///    let client = SiriusClient::new(NODE_URL, Client::new());
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
            hyper::Method::POST,
            TRANSACTIONS_STATUS_ROUTE.to_string(),
        );

        req = req.with_body_param(transaction_hashes);

        let dto: Vec<TransactionStatusDto> = req.execute(self.0).await?;

        let statuses: TransactionsStatus = dto.into_iter()
            .map(move |status_dto|
                {
                    status_dto.to_struct()
                }
            ).collect();

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
    ///use hyper::Client;
    ///use xpx_chain_sdk::sirius_client::SiriusClient;
    ///
    ///const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";
    ///const HASH: &str = "130171141CAE9D9ED6F62FD47CC316631986BBACD6B3D63930A9C46ED1ED764F";
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///
    ///    let client = SiriusClient::new(NODE_URL, Client::new());
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
    pub async fn get_transaction(self, transaction_id: &str) -> Result<Box<dyn Transaction>>
    {
        let mut req = __internal_request::Request::new(
            Method::GET,
            TRANSACTION_ROUTE.to_string(),
        );

        req = req.with_path_param("transactionId".to_string(), transaction_id.to_string())
            .is_transaction();

        let version: Box<dyn TransactionDto> = req.execute(self.0).await?;

        Ok(version.to_struct()?)
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
    ///use hyper::Client;
    ///use xpx_chain_sdk::sirius_client::SiriusClient;
    ///
    ///const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";
    ///const HASH_A: &str = "130171141CAE9D9ED6F62FD47CC316631986BBACD6B3D63930A9C46ED1ED764F";
    ///const HASH_B: &str = "5EC5C0E766B3DF81FBAD0E4FD794828002763905FEDC47208520E90FBED783B4";
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///
    ///    let client = SiriusClient::new(NODE_URL, Client::new());
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

        let mut req = __internal_request::Request::new(
            Method::POST,
            TRANSACTIONS_ROUTE.to_string(),
        );

        req = req.with_body_param(ids).is_transaction_vec();

        let dto: Vec<Box<dyn TransactionDto>> = req.execute(self.0).await?;

        let mut transactions_info: Transactions = vec![];
        for transaction_dto in dto.into_iter() {
            transactions_info.push(transaction_dto.to_struct()?);
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
    /// use hyper::Client;
    ///
    ///use xpx_chain_sdk::sirius_client::SiriusClient;
    ///use xpx_chain_sdk::{
    ///    account::{Account, Address},
    ///    message::PlainMessage,
    ///    mosaic::Mosaic,
    ///    network::PUBLIC_TEST,
    ///    transaction::{Deadline, TransferTransaction}
    ///};
    ///
    ///const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";
    ///const PRIVATE_KEY: &str = "5D3E959EB0CD69CC1DB6E9C62CB81EC52747AB56FA740CF18AACB5003429AD2E";
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///
    ///    let client = SiriusClient::new(NODE_URL, Client::new());
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
    pub async fn announce(self, transaction_signed: &SignedTransaction) -> Result<AnnounceTransactionInfo> {
        let mut req = __internal_request::Request::new(
            Method::PUT,
            "/transaction".to_string(),
        );

        req = req.with_body_param(transaction_signed);

        req.execute(self.0).await
    }

    pub async fn announce_partial(self, signed_transaction: &SignedTransaction) -> Result<AnnounceTransactionInfo> {
        let mut req = __internal_request::Request::new(
            Method::PUT,
            ANNOUNCE_AGGREGATE_ROUTE.to_string(),
        );

        req = req.with_body_param(signed_transaction);

        req.execute(self.0).await
    }

    pub async fn announce_cosignature(self, cosignature: String) -> Result<AnnounceTransactionInfo> {
        let mut req = __internal_request::Request::new(
            Method::PUT,
            ANNOUNCE_AGGREGATE_COSIGNATURE_ROUTE.to_string(),
        );
        req.with_body_param(cosignature);

        unimplemented!()

//        req.execute(self.client).await
    }
}

#[derive(Debug, Deserialize)]
pub struct AnnounceTransactionInfo {
    pub message: String,
}

impl Display for AnnounceTransactionInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f, "{}", &self.message
        )
    }
}
