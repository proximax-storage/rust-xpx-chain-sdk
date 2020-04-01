use crate::models::account::PublicAccount;
use crate::models::mosaic::{Mosaic, MosaicId};
use crate::models::transaction::Height;
use crate::models::uint_64::Uint64;
use num_enum::IntoPrimitive;
use std::collections::HashMap;

type OfferInfoDTOs = Vec<OfferInfo>;

#[derive(Debug, Clone, PartialEq, Serialize, Copy, IntoPrimitive, Eq, Hash)]
#[repr(u8)]
pub enum OfferType {
    SellOffer,
    BuyOffer,
    UnknownType,
}

impl OfferType {
    pub fn value(self) -> u8 {
        self.into()
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self)
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

#[derive(Debug, Serialize)]
pub struct ExchangeInfo {
    pub exchange: Exchange,
}

impl core::fmt::Display for ExchangeInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Exchange {
    pub owner: String,
    pub buy_offers: OfferInfoDTOs,
    pub sell_offers: OfferInfoDTOs,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferInfo {
    pub mosaic: Mosaic,
    pub initial_amount: Uint64,
    pub initial_cost: Uint64,
    pub deadline: Height,
}

impl core::fmt::Display for OfferInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferIdInfo {
    pub mosaic_id: MosaicId,
    pub offer_info: OfferInfo,
}

impl core::fmt::Display for OfferIdInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}

impl UserExchangeInfo {
    pub fn get_sell_offer(&self) -> Option<&OfferIdInfos> {
        self.offers.get(&OfferType::SellOffer)
    }

    pub fn get_buy_offer(&self) -> Option<&OfferIdInfos> {
        self.offers.get(&OfferType::BuyOffer)
    }
}

impl core::fmt::Display for UserExchangeInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}
