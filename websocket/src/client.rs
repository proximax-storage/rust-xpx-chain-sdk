// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use std::thread::sleep;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream};

use crate::model::{RouterPath, SubscribeDto, WsConnectionResponse, PATH_BLOCK, PATH_STATUS, PATH_CONFIRMED_ADDED, PATH_UNCONFIRMED_ADDED, PATH_UNCONFIRMED_REMOVED, PATH_PARTIAL_ADDED, PATH_PARTIAL_REMOVED, PATH_COSIGNATURE};

pub type AutoStream<S> = S;

pub struct SiriusWebsocketClient {
    uid: WsConnectionResponse,
    conn: WebSocketStream<AutoStream<tokio::net::TcpStream>>,
    handlers: Vec<fn(sdk::account::Address)>
}

impl SiriusWebsocketClient {
    pub async fn add_block_handlers(&mut self, handler: fn(sdk::account::Address)) -> super::Result<()> {
        self.handlers.push(handler);
        self.publish_subscribe_message(PATH_BLOCK).await
    }

    pub async fn add_confirmed_added_handlers(&mut self, address: sdk::account::Address) -> super::Result<()> {
        let path = PATH_CONFIRMED_ADDED.replace("{address}", &address.address);

        self.publish_subscribe_message(&path).await
    }

    pub async fn add_unconfirmed_added_handlers(&mut self, address: sdk::account::Address) -> super::Result<()> {
        let path = PATH_UNCONFIRMED_ADDED.replace("{address}", &address.address);
        self.publish_subscribe_message(&path).await
    }

    pub async fn add_unconfirmed_removed_handlers(&mut self, address: sdk::account::Address) -> super::Result<()> {
        let path = PATH_UNCONFIRMED_REMOVED.replace("{address}", &address.address);
        self.publish_subscribe_message(&path).await
    }

    pub async fn add_partial_added_handlers(&mut self, address: sdk::account::Address) -> super::Result<()> {
        let path = PATH_PARTIAL_ADDED.replace("{address}", &address.address);
        self.publish_subscribe_message(&path).await
    }

    pub async fn add_partial_removed_handlers(&mut self, address: sdk::account::Address) -> super::Result<()> {
        let path = PATH_PARTIAL_REMOVED.replace("{address}", &address.address);
        self.publish_subscribe_message(&path).await
    }

    pub async fn add_cosignature_handlers(&mut self, address: sdk::account::Address) -> super::Result<()> {
        let path = PATH_COSIGNATURE.replace("{address}", &address.address);
        self.publish_subscribe_message(&path).await
    }

    pub async fn add_status_handlers(&mut self, address: sdk::account::Address, handler: sdk::transaction::TransactionStatus) -> super::Result<()> {
        let path = PATH_STATUS.replace("{address}", &address.address);
        self.handlers.push(handler);
        self.publish_subscribe_message(&path).await
    }
}

impl SiriusWebsocketClient {
    pub async fn new(url: &str) -> super::Result<SiriusWebsocketClient>
    {
        let (mut conn, _) = connect_async(url).await.expect("Failed to connect");

        let msg = conn.next().await.expect("Can't fetch case count")?;

        let rwa_uid = msg.into_text()?;

        let uid: WsConnectionResponse = serde_json::from_str(&rwa_uid).unwrap();

        Ok(SiriusWebsocketClient { uid, conn, handlers: vec![] })
    }

    pub fn uid(&self) -> WsConnectionResponse {
        self.uid.to_owned()
    }

    pub async fn close(&mut self) -> super::Result<()> {
        self.conn.close(None).await
    }

    async fn publish_subscribe_message(&mut self, path: &RouterPath) -> super::Result<()> {
        let dto = SubscribeDto {
            uid: self.uid.uid.to_owned(),
            subscribe: path.to_string(),
        };

        let msg = serde_json::to_string(&dto).unwrap_or_default();

        self.conn.send(Message::text(msg)).await
    }

    pub async fn listen(&mut self) {
        while let Some(msg) = self.conn.next().await {
            let msg = msg.unwrap();
            if msg.is_text() {
                let algo = sdk::account::Address::from_raw("VCAEM4A2O3FDANHJICR5UVQUHIB3AOQEUO7L6QQN");
                self.handlers[0](algo.unwrap());
                // println!("{:?}", msg);
            }
        }
    }
}