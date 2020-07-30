/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use ::std::collections::HashMap;

use crate::{
    helpers::has_bits,
    models::{
        account::PublicAccount,
        exchange::{
            AddOffer, ExchangeConfirmation, Offer, OfferIdInfo, OfferIdInfos, OfferInfo, OfferType,
            OfferType::{BuyOffer, SellOffer},
            RemoveOffer, UserExchangeInfo,
        },
        mosaic::{Mosaic, MosaicId},
        namespace::{NamespaceId, NAMESPACE_BIT},
        network::NetworkType,
        transaction::{
            AddExchangeOfferTransaction, ExchangeOfferTransaction, RemoveExchangeOfferTransaction,
            Transaction,
        },
        AssetId, Result,
    },
};

use super::{AbstractTransactionDto, TransactionDto, TransactionMetaDto, Uint64Dto};

pub(crate) type OfferInfoDTOs = Vec<OfferInfoDto>;
type AddOfferDTOs = Vec<AddOfferDto>;
type RemoveOfferDTOs = Vec<RemoveOfferDto>;
type ConfirmationOfferDTOs = Vec<ConfirmationOffer>;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OfferDTO {
    mosaic_id: Uint64Dto,
    mosaic_amount: Uint64Dto,
    cost: Uint64Dto,
    r#type: u8,
}

impl OfferDTO {
    fn compact(&self) -> Offer {
        let asset_id = self.mosaic_id.compact();
        Offer {
            r#type: OfferType::from(self.r#type),
            mosaic: Mosaic {
                asset_id: Box::new(MosaicId::from(asset_id)),
                amount: self.mosaic_amount.compact(),
            },
            cost: self.cost.compact(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RemoveOfferDto {
    mosaic_id: Uint64Dto,
    offer_type: u8,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AddOfferDto {
    #[serde(flatten)]
    offer_dto: OfferDTO,
    duration: Uint64Dto,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ConfirmationOffer {
    #[serde(flatten)]
    offer_dto: OfferDTO,
    owner: String,
}

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
    pub(crate) fn compact(&self, network_type: NetworkType) -> Result<UserExchangeInfo> {
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
                mosaic_id: id.as_mosaic_id(),
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
                mosaic_id: id.as_mosaic_id(),
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
    pub owner: Option<String>,
}

impl OfferInfoDto {
    pub(crate) fn compact(&self, owner: PublicAccount) -> crate::Result<OfferInfo> {
        let dto = self.to_owned();
        let mosaic = Mosaic::new(
            MosaicId::from(dto.mosaic_id.compact()),
            dto.amount.compact().as_u64(),
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

/// AddExchangeOfferTransactionDto.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AddExchangeOfferTransactionDto {
    #[serde(flatten)]
    r#abstract: AbstractTransactionDto,
    pub offers: AddOfferDTOs,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AddExchangeOfferTransactionInfoDto {
    pub meta: TransactionMetaDto,
    pub transaction: AddExchangeOfferTransactionDto,
}

#[typetag::serde]
impl TransactionDto for AddExchangeOfferTransactionInfoDto {
    fn compact(&self) -> Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();
        let info = self.meta.compact()?;

        let abs_transaction = dto.r#abstract.compact(info)?;

        let offers: Vec<AddOffer> = dto
            .offers
            .into_iter()
            .map(|offer| {
                let _offer = offer.offer_dto.compact();
                AddOffer {
                    offer: _offer,
                    duration: offer.duration.compact().as_u64(),
                }
            })
            .collect();

        Ok(Box::new(AddExchangeOfferTransaction {
            abs_transaction,
            offers,
        }))
    }
}

/// AddExchangeOfferTransactionDto.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ExchangeOfferTransactionDto {
    #[serde(flatten)]
    r#abstract: AbstractTransactionDto,
    pub offers: ConfirmationOfferDTOs,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ExchangeOfferTransactionInfoDto {
    pub meta: TransactionMetaDto,
    pub transaction: ExchangeOfferTransactionDto,
}

#[typetag::serde]
impl TransactionDto for ExchangeOfferTransactionInfoDto {
    fn compact(&self) -> Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();
        let info = self.meta.compact()?;

        let abs_transaction = dto.r#abstract.compact(info)?;

        let confirmations: Vec<ExchangeConfirmation> = dto
            .offers
            .into_iter()
            .map(|offer| {
                let _offer = offer.offer_dto.compact();
                ExchangeConfirmation {
                    offer: _offer,
                    owner: PublicAccount::from_public_key(
                        &offer.owner,
                        abs_transaction.network_type,
                    )
                    .unwrap(),
                }
            })
            .collect();

        Ok(Box::new(ExchangeOfferTransaction {
            abs_transaction,
            confirmations,
        }))
    }
}

/// AddExchangeOfferTransactionDto.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RemoveExchangeOfferTransactionDto {
    #[serde(flatten)]
    r#abstract: AbstractTransactionDto,
    pub offers: RemoveOfferDTOs,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RemoveExchangeOfferTransactionInfoDto {
    pub meta: TransactionMetaDto,
    pub transaction: RemoveExchangeOfferTransactionDto,
}

#[typetag::serde]
impl TransactionDto for RemoveExchangeOfferTransactionInfoDto {
    fn compact(&self) -> Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();
        let info = self.meta.compact()?;

        let abs_transaction = dto.r#abstract.compact(info)?;

        let offers: Vec<RemoveOffer> = dto
            .offers
            .into_iter()
            .map(|offer| {
                let id = offer.mosaic_id.compact();

                let asset_id: Box<dyn AssetId> = if has_bits(id.as_u64(), NAMESPACE_BIT) {
                    Box::new(NamespaceId::from(id))
                } else {
                    Box::new(MosaicId::from(id))
                };

                RemoveOffer {
                    r#type: OfferType::from(offer.offer_type),
                    asset_id,
                }
            })
            .collect();

        Ok(Box::new(RemoveExchangeOfferTransaction {
            abs_transaction,
            offers,
        }))
    }
}
