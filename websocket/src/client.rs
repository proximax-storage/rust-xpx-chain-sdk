// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use {
    bytes::Bytes,
    downcast_rs::Downcast,
    futures_util::{SinkExt, StreamExt},
    serde_json::Value,
    std::{borrow::Cow, collections::HashMa},
    tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream},
    url::Url,
};

use crate::{
    error::Error,
    model::{
        RouterPath, SubscribeDto, WsConnectionResponse, WsSubscribeDto, PATH_BLOCK,
        PATH_CONFIRMED_ADDED, PATH_COSIGNATURE, PATH_PARTIAL_ADDED, PATH_PARTIAL_REMOVED,
        PATH_STATUS, PATH_UNCONFIRMED_ADDED, PATH_UNCONFIRMED_REMOVED,
    },
    HandlerBlock, HandlerConfirmedAdd, HandlerCosignature, HandlerPartialAdd, HandlerPartialRemove,
    HandlerStatus, HandlerUnconfirmedAdd, HandlerUnconfirmedRemoved, WsBlockInfoDto,
    WsPartialRemoveDto, WsStatusInfoDto, WsUnconfirmedRemovedDto,
};

pub type AutoStream<S> = S;

pub trait Handler: Send + Downcast {}

impl_downcast!(Handler);

pub struct SiriusWebsocketClient {
    uid: WsConnectionResponse,
    conn: WebSocketStream<AutoStream<tokio::net::TcpStream>>,
    pub handlers: HashMap<String, Box<dyn Handler>>,
}

impl SiriusWebsocketClient {
    pub async fn add_block_handlers<F>(&mut self, handler_fn: F) -> super::Result<()>
    where
        F: Fn(sdk::blockchain::BlockInfo) -> bool + Send + 'static,
    {
        let handler = HandlerBlock {
            handler: Box::new(handler_fn),
        };

        self.publish_subscribe_message(PATH_BLOCK).await?;
        self.handlers
            .insert(PATH_BLOCK.to_string(), Box::new(handler));
        Ok(())
    }

    pub async fn add_status_handlers<F>(
        &mut self,
        address: &sdk::account::Address,
        handler_fn: F,
    ) -> super::Result<()>
    where
        F: Fn(sdk::transaction::TransactionStatus) -> bool + Send + 'static,
    {
        let handler = HandlerStatus {
            handler: Box::new(handler_fn),
        };

        self.publish_subscribe_message(&path_parse_address(PATH_STATUS.to_string(), address))
            .await?;
        self.handlers
            .insert(PATH_STATUS.to_string(), Box::new(handler));
        Ok(())
    }

    pub async fn add_confirmed_added_handlers<F>(
        &mut self,
        address: &sdk::account::Address,
        handler_fn: F,
    ) -> super::Result<()>
    where
        F: Fn(Box<dyn sdk::transaction::Transaction>) -> bool + Send + 'static,
    {
        let handler = HandlerConfirmedAdd {
            handler: Box::new(handler_fn),
        };

        self.publish_subscribe_message(&path_parse_address(
            PATH_CONFIRMED_ADDED.to_string(),
            address,
        ))
        .await?;
        self.handlers
            .insert(PATH_CONFIRMED_ADDED.to_string(), Box::new(handler));
        Ok(())
    }

    pub async fn add_unconfirmed_removed_handlers<F>(
        &mut self,
        address: &sdk::account::Address,
        handler_fn: F,
    ) -> super::Result<()>
    where
        F: Fn(sdk::transaction::TransactionInfo) -> bool + Send + 'static,
    {
        let handler = HandlerUnconfirmedRemoved {
            handler: Box::new(handler_fn),
        };

        self.publish_subscribe_message(&path_parse_address(
            PATH_UNCONFIRMED_REMOVED.to_string(),
            address,
        ))
        .await?;
        self.handlers
            .insert(PATH_UNCONFIRMED_REMOVED.to_string(), Box::new(handler));
        Ok(())
    }

    pub async fn add_unconfirmed_added_handlers<F>(
        &mut self,
        address: &sdk::account::Address,
        handler_fn: F,
    ) -> super::Result<()>
    where
        F: Fn(Box<dyn sdk::transaction::Transaction>) -> bool + Send + 'static,
    {
        let handler = HandlerUnconfirmedAdd {
            handler: Box::new(handler_fn),
        };

        self.publish_subscribe_message(&path_parse_address(
            PATH_UNCONFIRMED_ADDED.to_string(),
            address,
        ))
        .await?;
        self.handlers
            .insert(PATH_UNCONFIRMED_ADDED.to_string(), Box::new(handler));
        Ok(())
    }

    pub async fn add_partial_added_handlers<F>(
        &mut self,
        address: &sdk::account::Address,
        handler_fn: F,
    ) -> super::Result<()>
    where
        F: Fn(sdk::transaction::AggregateTransaction) -> bool + Send + 'static,
    {
        let handler = HandlerPartialAdd {
            handler: Box::new(handler_fn),
        };

        self.publish_subscribe_message(&path_parse_address(
            PATH_PARTIAL_ADDED.to_string(),
            address,
        ))
        .await?;
        self.handlers
            .insert(PATH_PARTIAL_ADDED.to_string(), Box::new(handler));
        Ok(())
    }

    pub async fn add_partial_removed_handlers<F>(
        &mut self,
        address: &sdk::account::Address,
        handler_fn: F,
    ) -> super::Result<()>
    where
        F: Fn(sdk::transaction::TransactionInfo) -> bool + Send + 'static,
    {
        let handler = HandlerUnconfirmedRemoved {
            handler: Box::new(handler_fn),
        };

        self.publish_subscribe_message(&path_parse_address(
            PATH_PARTIAL_REMOVED.to_string(),
            address,
        ))
        .await?;
        self.handlers
            .insert(PATH_PARTIAL_REMOVED.to_string(), Box::new(handler));
        Ok(())
    }

    pub async fn add_cosignature_handlers<F>(
        &mut self,
        address: &sdk::account::Address,
        handler_fn: F,
    ) -> super::Result<()>
    where
        F: Fn(sdk::multisig::CosignatureInfo) -> bool + Send + 'static,
    {
        let handler = HandlerCosignature {
            handler: Box::new(handler_fn),
        };

        self.publish_subscribe_message(&path_parse_address(PATH_COSIGNATURE.to_string(), address))
            .await?;
        self.handlers
            .insert(PATH_COSIGNATURE.to_string(), Box::new(handler));
        Ok(())
    }
}

impl SiriusWebsocketClient {
    pub async fn new(url: &str) -> super::Result<SiriusWebsocketClient> {
        let scheme_str = convert_to_ws_url(url)?;

        let (mut conn, _) = connect_async(scheme_str).await?;

        let msg = conn.next().await.expect("Can't fetch case count")?;

        let rwa_uid = msg.into_text()?;

        let uid: WsConnectionResponse = serde_json::from_str(&rwa_uid)?;

        Ok(SiriusWebsocketClient {
            uid,
            conn,
            handlers: HashMap::new(),
        })
    }

    pub fn uid(&self) -> String {
        self.uid.uid.to_string()
    }

    pub async fn close(&mut self) -> super::Result<()> {
        Ok(self.conn.close(None).await?)
    }

    async fn publish_subscribe_message(&mut self, path: &RouterPath) -> super::Result<()> {
        let dto = SubscribeDto {
            uid: self.uid.uid.to_owned(),
            subscribe: path.to_string(),
        };

        let msg = serde_json::to_string(&dto)?;

        Ok(self.conn.send(Message::text(msg)).await?)
    }

    pub async fn listen(&mut self) -> super::Result<()> {
        while let Some(msg) = self.conn.next().await {
            let msg =
                msg.map_err(|e| failure::err_msg(format!("Error on server stream: {:?}", e)))?;

            if msg.is_text() {
                let msg_string = msg.to_string();
                let channel_name = get_channel_name(&msg_string)?;

                if let Some(base) = self.handlers.get(&channel_name) {
                    if let Some(handler_info) = base.downcast_ref::<HandlerBlock>() {
                        let channel = get_channel_data::<WsBlockInfoDto>(&msg_string, false)?;
                        if (handler_info.handler)(channel.compact()) {
                            break;
                        }
                    } else if let Some(handler_info) = base.downcast_ref::<HandlerStatus>() {
                        let channel = get_channel_data::<WsStatusInfoDto>(&msg_string, false)?;
                        if (handler_info.handler)(channel.compact()) {
                            break;
                        }
                    } else if let Some(handler_info) = base.downcast_ref::<HandlerConfirmedAdd>() {
                        let channel =
                            get_channel_data::<Box<dyn api::TransactionDto>>(&msg_string, true)?;
                        if (handler_info.handler)(channel.compact()?) {
                            break;
                        }
                    } else if let Some(handler_info) = base.downcast_ref::<HandlerUnconfirmedAdd>()
                    {
                        let channel =
                            get_channel_data::<Box<dyn api::TransactionDto>>(&msg_string, true)?;
                        if (handler_info.handler)(channel.compact()?) {
                            break;
                        }
                    } else if let Some(handler_info) =
                        base.downcast_ref::<HandlerUnconfirmedRemoved>()
                    {
                        let channel =
                            get_channel_data::<WsUnconfirmedRemovedDto>(&msg_string, false)?;
                        if (handler_info.handler)(channel.compact()) {
                            break;
                        }
                    } else if let Some(handler_info) = base.downcast_ref::<HandlerPartialAdd>() {
                        let channel =
                            get_channel_data::<Box<dyn api::TransactionDto>>(&msg_string, true)?;
                        let tx = channel.compact()?;
                        let aggregate = tx
                            .downcast::<sdk::transaction::AggregateTransaction>()
                            .map_err(|_| {
                                failure::err_msg(
                                    sdk::errors_const::ERR_INVALID_AGGREGATE_TRANSACTION,
                                )
                            })?;
                        if (handler_info.handler)(*aggregate) {
                            break;
                        }
                    } else if let Some(handler_info) = base.downcast_ref::<HandlerPartialRemove>() {
                        let channel = get_channel_data::<WsPartialRemoveDto>(&msg_string, false)?;
                        if (handler_info.handler)(channel.compact()) {
                            break;
                        }
                    } else if let Some(handler_info) = base.downcast_ref::<HandlerCosignature>() {
                        let channel =
                            get_channel_data::<sdk::multisig::CosignatureInfo>(&msg_string, false)?;
                        if (handler_info.handler)(channel) {
                            break;
                        }
                    }
                };
            }
        }
        self.close().await
    }
}

fn convert_to_ws_url(url: &str) -> super::Result<Url> {
    let scheme_vec: Vec<&str> = url.split(":").collect();

    let scheme_str = match scheme_vec[0] {
        "https" => "wss",
        "http" => "ws",
        _ => return Err(Error::Url(Cow::from("invalid scheme"))),
    };

    let mut url = url.replace(scheme_vec[0], scheme_str);
    url.push_str("/ws");

    Url::parse(&url).map_err(|e| Error::Url(Cow::from(e.to_string())))
}

fn path_parse_address(mut path: String, address: &sdk::account::Address) -> String {
    path.push_str("/");
    path.push_str(&address.address);
    path
}

fn get_channel_data<U>(msg: &str, is_tx: bool) -> super::Result<U>
where
    for<'de> U: serde::Deserialize<'de>,
{
    if is_tx {
        let map_dto = api::map_transaction_dto(Bytes::from(msg.to_string()))?;
        Ok(serde_json::from_str(&map_dto)?)
    } else {
        Ok(serde_json::from_str(msg)?)
    }
}

fn get_channel_name(msg: &str) -> super::Result<String> {
    let value_dto: Value = serde_json::from_str(msg)?;
    let res = value_dto["meta"]["channelName"].as_str().unwrap();
    Ok(res.to_string())
}
