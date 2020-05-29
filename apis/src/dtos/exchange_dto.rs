// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use ::std::collections::HashMap;

use sdk::{
    account::PublicAccount,
    exchange::{OfferIdInfo, OfferIdInfos, OfferInfo, OfferType, UserExchangeInfo},
    exchange::OfferType::{BuyOffer, SellOffer},
    mosaic::{Mosaic, MosaicId},
    network::NetworkType,
};

use super::Uint64Dto;

pub(crate) type OfferInfoDTOs = Vec<OfferInfoDto>;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ExchangeDto {
    owner: String,
    owner_address: String,
    buy_offers: OfferInfoDTOs,
    sell_offers: OfferInfoDTOs,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ExchangeInfoDto {
    exchange: ExchangeDto,
}

impl ExchangeInfoDto {
    pub(crate) fn compact(&self, network_type: NetworkType) -> crate::Result<UserExchangeInfo> {
        let dto = self.to_owned();
        let owner = PublicAccount::from_public_key(&dto.exchange.owner, network_type)?;

        let mut buy_offer: OfferIdInfos = vec![];
        let mut buy_offer_offers: Vec<OfferInfo> = vec![];
        for item in dto.exchange.buy_offers.into_iter() {
            buy_offer_offers.push(item.compact(owner.to_owned())?)
        }

        for offer_info in buy_offer_offers.into_iter() {
            let id = &offer_info.mosaic.asset_id;
            buy_offer.push(OfferIdInfo {
                mosaic_id: id.to_mosaic_id(),
                offer_info,
            });
        }

        let mut sell_offer: OfferIdInfos = vec![];
        let mut sell_offer_offers: Vec<OfferInfo> = vec![];
        for item in dto.exchange.sell_offers.into_iter() {
            sell_offer_offers.push(item.compact(owner.to_owned())?)
        }

        for offer_info in sell_offer_offers.into_iter() {
            let id = &offer_info.mosaic.asset_id;
            sell_offer.push(OfferIdInfo {
                mosaic_id: id.to_mosaic_id(),
                offer_info,
            });
        }

        let mut offers_map: HashMap<OfferType, OfferIdInfos> = HashMap::new();
        offers_map.insert(BuyOffer, buy_offer);
        offers_map.insert(SellOffer, sell_offer);

        Ok(UserExchangeInfo {
            owner,
            offers: offers_map,
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OfferInfoDto {
    mosaic_id: Uint64Dto,
    amount: Uint64Dto,
    initial_amount: Uint64Dto,
    initial_cost: Uint64Dto,
    price: u64,
    residual_cost: Option<Uint64Dto>,
    deadline: Uint64Dto,
    owner: Option<String>
}

impl OfferInfoDto {
    pub(crate) fn compact(&self, owner: PublicAccount) -> crate::Result<OfferInfo> {
        let dto = self.to_owned();
        let mosaic = Mosaic::new(
            MosaicId::from(dto.mosaic_id.compact()),
            dto.amount.compact(),
        );

        Ok(OfferInfo {
            owner,
            mosaic,
            price_denominator: dto.initial_amount.compact(),
            price_numerator: dto.initial_cost.compact(),
            deadline: dto.deadline.compact(),
        })
    }
}
