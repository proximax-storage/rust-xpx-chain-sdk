/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {num_enum::IntoPrimitive, std::collections::HashMap};

use crate::models::{
    account::PublicAccount,
    mosaic::{Mosaic, MosaicId},
    transaction::Height,
    uint_64::Uint64,
};

pub type OfferInfos = Vec<OfferInfo>;

#[derive(Debug, Clone, PartialEq, Serialize, Copy, IntoPrimitive, Eq, Hash)]
#[repr(u8)]
pub enum OfferType {
    #[serde(rename = "sell")]
    SellOffer,
    #[serde(rename = "buy")]
    BuyOffer,
    #[serde(rename = "unknown")]
    UnknownType,
}

impl OfferType {
    pub fn value(self) -> u8 {
        self.into()
    }

    pub fn counter_offer(self) -> Self {
        match self {
            OfferType::SellOffer => OfferType::BuyOffer,
            OfferType::BuyOffer => OfferType::SellOffer,
            _ => OfferType::UnknownType,
        }
    }
}

impl From<u8> for OfferType {
    fn from(num: u8) -> Self {
        match num {
            0 => OfferType::SellOffer,
            1 => OfferType::BuyOffer,
            _ => OfferType::UnknownType,
        }
    }
}

impl core::fmt::Display for OfferType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or_default())
    }
}

#[derive(Debug, Serialize)]
pub struct ExchangeInfo {
    pub exchange: Exchange,
}

impl core::fmt::Display for ExchangeInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{:?}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Exchange {
    pub owner: String,
    pub buy_offers: OfferInfos,
    pub sell_offers: OfferInfos,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferInfo {
    pub owner: PublicAccount,
    pub mosaic: Mosaic,
    pub price_denominator: Uint64,
    pub price_numerator: Uint64,
    pub deadline: Height,
}

impl core::fmt::Display for OfferInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}

#[derive(Debug, Serialize)]
pub struct UserExchangeInfo {
    pub owner: PublicAccount,
    pub offers: HashMap<OfferType, OfferIdInfos>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferTypeInfo {
    pub mosaic_id: MosaicId,
    pub offer_info: OfferInfo,
}

pub type OfferIdInfos = Vec<OfferIdInfo>;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferIdInfo {
    pub mosaic_id: MosaicId,
    pub offer_info: OfferInfo,
}

impl core::fmt::Display for OfferIdInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}

impl UserExchangeInfo {
    pub fn get_sell_offer(&self) -> OfferIdInfos {
        if let Some(o) = self.offers.get(&OfferType::SellOffer) {
            o.to_vec()
        } else {
            vec![]
        }
    }

    pub fn get_buy_offer(&self) -> OfferIdInfos {
        if let Some(o) = self.offers.get(&OfferType::BuyOffer) {
            o.to_vec()
        } else {
            vec![]
        }
    }
}

impl core::fmt::Display for UserExchangeInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}
