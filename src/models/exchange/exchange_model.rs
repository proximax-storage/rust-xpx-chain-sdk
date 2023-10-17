/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::fmt;

use {num_enum::IntoPrimitive, std::collections::HashMap};

use crate::models::{
    account::PublicAccount,
    mosaic::{Mosaic, MosaicId, UnresolvedMosaicId},
    transaction::{Amount, Height},
};

pub type OfferInfos = Vec<OfferInfo>;
pub type OfferIdInfos = Vec<OfferIdInfo>;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Copy, IntoPrimitive, Eq, Hash)]
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
        use OfferType::*;
        match self {
            SellOffer => BuyOffer,
            BuyOffer => SellOffer,
            _ => UnknownType,
        }
    }
}

impl From<u8> for OfferType {
    fn from(num: u8) -> Self {
        use OfferType::*;
        match num {
            0 => SellOffer,
            1 => BuyOffer,
            _ => UnknownType,
        }
    }
}

impl fmt::Display for OfferType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or_default())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Offer {
    pub r#type: OfferType,
    pub mosaic: Mosaic,
    pub cost: Amount,
}

impl Offer {
    pub fn create(offer_type: OfferType, mosaic: Mosaic, cost: Amount) -> Self {
        Self { r#type: offer_type, mosaic, cost }
    }
}

impl fmt::Display for Offer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap_or_default())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddOffer {
    #[serde(flatten)]
    pub offer: Offer,
    pub duration: u64,
}

impl AddOffer {
    pub fn create(offer: Offer, duration: u64) -> Self {
        Self { offer, duration }
    }
}

impl fmt::Display for AddOffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap_or_default())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveOffer {
    pub r#type: OfferType,
    pub asset_id: Box<dyn UnresolvedMosaicId>,
}

impl RemoveOffer {
    pub fn create(offer_type: OfferType, asset_id: impl UnresolvedMosaicId + 'static) -> Self {
        Self { r#type: offer_type, asset_id: Box::new(asset_id) }
    }
}

impl fmt::Display for RemoveOffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferInfo {
    pub owner: PublicAccount,
    pub mosaic: Mosaic,
    pub price_denominator: u64,
    pub price_numerator: u64,
    pub deadline: Height,
}

impl fmt::Display for OfferInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap_or_default())
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferIdInfo {
    pub mosaic_id: MosaicId,
    pub offer_info: OfferInfo,
}

impl fmt::Display for OfferIdInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap_or_default())
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferTypeInfo {
    pub mosaic_id: MosaicId,
    pub offer_info: OfferInfo,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Exchange {
    pub owner: String,
    pub buy_offers: OfferInfos,
    pub sell_offers: OfferInfos,
}

impl fmt::Display for Exchange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap_or_default())
    }
}

#[derive(Debug, Serialize)]
pub struct ExchangeInfo {
    pub exchange: Exchange,
}

impl fmt::Display for ExchangeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", serde_json::to_string_pretty(self).unwrap_or_default())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeConfirmation {
    #[serde(flatten)]
    pub offer: Offer,
    pub owner: PublicAccount,
}

impl ExchangeConfirmation {
    pub fn create(offer: Offer, owner: PublicAccount) -> Self {
        Self { offer, owner }
    }
}

impl fmt::Display for ExchangeConfirmation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap_or_default())
    }
}

#[derive(Debug, Serialize)]
pub struct UserExchangeInfo {
    pub owner: PublicAccount,
    pub offers: HashMap<OfferType, OfferIdInfos>,
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

impl fmt::Display for UserExchangeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap_or_default())
    }
}
