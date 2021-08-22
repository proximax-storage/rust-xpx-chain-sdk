/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::{
    api::mosaic_properties,
    models::{
        errors_const,
        mosaic::{Mosaic, MosaicId, MosaicInfo, MosaicNames, MosaicNonce, MosaicSupplyType},
        Result,
        transaction::{
            MetadataMosaicTransaction, MosaicDefinitionTransaction, MosaicSupplyChangeTransaction,
            Transaction,
        },
    },
};

use super::{
    AbstractTransactionDto, ModifyMetadataTransactionDto, TransactionDto, TransactionMetaDto,
    Uint64Dto,
};

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct MosaicDto {
    #[serde(rename = "id")]
    id: Uint64Dto,
    #[serde(rename = "amount")]
    amount: Uint64Dto,
}

impl MosaicDto {
    pub fn compact(&self) -> Mosaic {
        let mosaic_id = MosaicId::from(self.id.compact());
        let amount = self.amount.compact();
        Mosaic::new(mosaic_id, amount.as_u64())
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MosaicInfoDto {
    #[serde(rename = "meta")]
    meta: MosaicMetaDto,
    #[serde(rename = "mosaic")]
    mosaic: MosaicDefinitionDto,
}

impl MosaicInfoDto {
    pub fn compact(&self) -> crate::Result<MosaicInfo> {
        ensure!(
            !self.mosaic.properties.is_empty(),
            errors_const::ERR_INVALID_MOSAIC_PROPERTIES
        );

        let mosaic_id = MosaicId::from(self.mosaic.mosaic_id.compact());

        let properties = mosaic_properties(&self.mosaic.properties)?;

        Ok(MosaicInfo::new(
            mosaic_id,
            self.mosaic.supply.compact(),
            self.mosaic.height.compact(),
            (&self.mosaic.owner).parse()?,
            self.mosaic.revision,
            properties,
        ))
    }
}

#[derive(Serialize, Deserialize)]
struct MosaicMetaDto {
    #[serde(rename = "id")]
    id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicDefinitionTransactionInfoDto {
    meta: TransactionMetaDto,
    transaction: MosaicDefinitionTransactionDto,
}

/// MosaicDefinitionTransactionDto : Transaction that creates a new mosaic.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicDefinitionTransactionDto {
    #[serde(flatten)]
    r#abstract: AbstractTransactionDto,
    /// Random nonce used to generate the mosaic id.
    mosaic_nonce: u32,
    mosaic_id: Uint64Dto,
    properties: Vec<MosaicPropertyDto>,
}

#[typetag::serde]
impl TransactionDto for MosaicDefinitionTransactionInfoDto {
    fn compact(&self) -> Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();

        let info = self.meta.compact()?;

        let abs_transaction = dto.r#abstract.compact(info)?;

        let properties = mosaic_properties(&dto.properties)?;

        Ok(Box::new(MosaicDefinitionTransaction {
            abs_transaction,
            properties,
            mosaic_nonce: MosaicNonce::from(dto.mosaic_nonce),
            mosaic_id: MosaicId::from(dto.mosaic_id.compact()),
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicDefinitionDto {
    mosaic_id: Uint64Dto,
    supply: Uint64Dto,
    height: Uint64Dto,
    owner: String,
    revision: usize,
    properties: Vec<MosaicPropertyDto>,
}

/// MosaicMetadataTransactionDto :
/// Transaction that addes metadata to mosaic.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicMetadataTransactionDto {
    #[serde(flatten)]
    metadata_transaction: ModifyMetadataTransactionDto,
    #[serde(rename = "metadataId")]
    mosaic_id: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicMetadataTransactionInfoDto {
    meta: TransactionMetaDto,
    transaction: MosaicMetadataTransactionDto,
}

#[typetag::serde]
impl TransactionDto for MosaicMetadataTransactionInfoDto {
    fn compact(&self) -> Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();

        let info = self.meta.compact()?;

        let metadata_transaction = dto.metadata_transaction.compact(info)?;

        let mosaic_id = MosaicId::from(dto.mosaic_id.compact());

        Ok(Box::new(MetadataMosaicTransaction {
            metadata_transaction,
            mosaic_id,
        }))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicNamesDto {
    mosaic_id: Uint64Dto,
    names: Vec<String>,
}

impl MosaicNamesDto {
    pub fn compact(&self) -> MosaicNames {
        MosaicNames::new(
            MosaicId::from(self.mosaic_id.compact()),
            (self.names).to_owned(),
        )
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct MosaicPropertyDto {
    pub(crate) id: u8,
    pub(crate) value: Uint64Dto,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicSupplyChangeTransactionInfoDto {
    meta: TransactionMetaDto,
    transaction: MosaicSupplyChangeTransactionDto,
}

#[typetag::serde]
impl TransactionDto for MosaicSupplyChangeTransactionInfoDto {
    fn compact(&self) -> Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();

        let info = self.meta.compact()?;

        let abs_transaction = dto.r#abstract.compact(info)?;

        Ok(Box::new(MosaicSupplyChangeTransaction {
            abs_transaction,
            supply_type: MosaicSupplyType::from(dto.direction),
            asset_id: Box::new(MosaicId::from(dto.mosaic_id.compact())),
            delta: dto.delta.compact(),
        }))
    }
}

/// MosaicSupplyChangeTransactionDto : Transaction to increase or decrease a mosaic’s supply.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicSupplyChangeTransactionDto {
    #[serde(flatten)]
    r#abstract: AbstractTransactionDto,
    mosaic_id: Uint64Dto,
    direction: u8,
    delta: Uint64Dto,
}
