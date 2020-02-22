use ::std::fmt::Debug;
use ::std::sync::Arc;
use std::any::Any;
use std::borrow::{Borrow, BorrowMut};
use std::fmt;
use std::rc::Rc;

use bytes::Buf;
use hyper::{client::connect::Connect, Method};
use serde::{de, Deserialize, Deserializer};
use serde::de::{IntoDeserializer, MapAccess, Unexpected, Visitor};
use serde::export::PhantomData;

use crate::{
    models::{
        account::{AccountInfo, AccountInfoDto},
        transaction::{
            SignedTransaction,
            Transaction,
            TransactionHashes,
            TransactionIds,
            TransactionStatus,
            TransactionStatusDto,
        },
        utils::is_hex,
    }};
use crate::models::transaction::TransactionDto;

use super::{
    internally::{valid_hash, valid_vec_hash, valid_vec_len},
    request as __internal_request,
    Result,
    sirius_client::ApiClient};

#[derive(Clone)]
pub struct TransactionRoutesApiClient<C: Connect> {
    client: Arc<ApiClient<C>>,
}

impl<C: Connect> TransactionRoutesApiClient<C> where
    C: Clone + Send + Sync + Debug + 'static
{
    pub fn new(client: Arc<ApiClient<C>>) -> Self {
        TransactionRoutesApiClient {
            client,
        }
    }
}

impl<C: Connect> TransactionRoutesApiClient<C> where
    C: Clone + Send + Sync + Debug + 'static,
{
    pub async fn get_transaction_status(self, hash: &str) -> Result<TransactionStatus> {
        valid_hash(hash)?;

        let mut req = __internal_request::Request::new(
            Method::GET,
            "/transaction/{hash}/status".to_string(),
        );

        req = req.with_path_param("hash".to_string(), hash.to_string());

        let dto: Result<TransactionStatusDto> = req.execute(self.client).await;

        Ok(dto?.to_struct())
    }

    pub async fn get_transactions_statuses(self, transaction_hashes: Vec<&str>) -> Result<(Vec<TransactionStatus>)> {
        valid_vec_len(&transaction_hashes)?;

        valid_vec_hash(&transaction_hashes)?;

        let hashes = TransactionHashes::from(transaction_hashes);

        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/transaction/statuses".to_string(),
        );

        req = req.with_body_param(hashes);

        let dto: Vec<TransactionStatusDto> = req.execute(self.client).await?;

        let mut statuses: Vec<TransactionStatus> = Vec::with_capacity(dto.len());
        for i in dto {
            let statuse = i;
            statuses.push(statuse.to_struct());
        }

        Ok(statuses)
    }

    pub async fn get_transaction(self, transaction_id: &str) -> Result<()>
    {
        let mut req = __internal_request::Request::new(
            Method::GET,
            "/transaction/{transactionId}".to_string(),
        );

        req = req.with_path_param("transactionId".to_string(), transaction_id.to_string())
            .is_transaction();

        let version: Box<dyn TransactionDto> = req.execute(self.client).await?;

        Ok(())
    }

    pub async fn announce_transaction(self, transaction_payload: &SignedTransaction) -> Result<AnnounceTransactionInfo> {
        let mut req = __internal_request::Request::new(
            Method::PUT,
            "/transaction".to_string(),
        );

        req = req.with_body_param(transaction_payload);

        req.execute(self.client).await
    }

    pub async fn announce_cosignature_transaction(self, cosignature: String) -> Result<AnnounceTransactionInfo> {
        let mut req = __internal_request::Request::new(
            Method::PUT,
            "/transaction/cosignature".to_string(),
        );
        req = req.with_body_param(cosignature);

        unimplemented!()

//        req.execute(self.client).await
    }

    pub async fn announce_partial_transaction(self, transaction_payload: &SignedTransaction) -> Result<(AnnounceTransactionInfo)> {
        let mut req = __internal_request::Request::new(
            Method::PUT,
            "/transaction/partial".to_string(),
        );

        req = req.with_body_param(transaction_payload);

        req.execute(self.client).await
    }

    pub async fn get_transactions(self, transaction_ids: Vec<&str>) -> Result<(Vec<&dyn Transaction>)> {
        let mut req = __internal_request::Request::new(
            Method::POST,
            "/transaction".to_string(),
        );

        req = req.with_body_param(transaction_ids);

        unimplemented!()
//        req.execute(self.client).await
    }
}

#[derive(Debug, Deserialize)]
pub struct AnnounceTransactionInfo {
    #[serde(rename = "message")]
    pub message: String,
}

impl fmt::Display for AnnounceTransactionInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f, "{}", &self.message
        )
    }
}
