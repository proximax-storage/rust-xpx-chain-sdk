use std::collections::HashMap;

use crate::models::account::PublicAccount;
use crate::models::exchange::OfferType::{BuyOffer, SellOffer};
use crate::models::exchange::{OfferIdInfo, OfferIdInfos, OfferInfo, OfferType, UserExchangeInfo};
use crate::models::mosaic::{Mosaic, MosaicId};
use crate::models::network::NetworkType;
use crate::models::uint_64::Uint64Dto;

type OfferInfoDTOs = Vec<OfferInfoDto>;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ExchangeDto {
    owner: String,
    buy_offers: OfferInfoDTOs,
    sell_offers: OfferInfoDTOs,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ExchangeInfoDto {
    exchange: ExchangeDto,
}

impl ExchangeInfoDto {
    pub(crate) fn to_struct(&self, network_type: NetworkType) -> crate::Result<UserExchangeInfo> {
        let mut dto = self.to_owned();
        let owner = PublicAccount::from_public_key(&dto.exchange.owner, network_type)?;

        let mut buy_offer: OfferIdInfos = vec![];
        let mut buy_offer_offers: Vec<OfferInfo> = vec![];
        for item in dto.exchange.buy_offers.into_iter() {
            buy_offer_offers.push(item.to_struct()?)
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
            sell_offer_offers.push(item.to_struct()?)
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
    deadline: Uint64Dto,
    price: u64,
}

impl OfferInfoDto {
    fn to_struct(&self) -> crate::Result<OfferInfo> {
        let dto = self.to_owned();
        let mosaic = Mosaic::new(
            MosaicId::from(dto.mosaic_id.to_struct()),
            dto.amount.to_struct(),
        );

        Ok(OfferInfo {
            mosaic,
            initial_amount: dto.initial_amount.to_struct(),
            initial_cost: dto.initial_cost.to_struct(),
            deadline: dto.deadline.to_struct(),
        })
    }
}
