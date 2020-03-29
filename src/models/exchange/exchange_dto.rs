use crate::models::uint_64::Uint64Dto;

type OfferInfoDTOs = Vec<OfferInfoDto>;

pub(crate) struct ExchangeInfoDto {
    exchange: ExchangeDto
}


pub(crate) struct ExchangeDto {
    owner: String,
    buy_offers: OfferInfoDTOs,
    sell_offers: OfferInfoDTOs
}

pub(crate) struct OfferInfoDto {
    mosaic_id: Uint64Dto,
    amount: Uint64Dto,
    price_numerator: Uint64Dto,
    price_denominator: Uint64Dto,
    deadline: Uint64Dto,
    owner: String,
    r#type: u8
}