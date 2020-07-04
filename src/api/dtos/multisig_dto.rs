/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::{
    account::{Address, PublicAccount},
    api::cosignatory_dto_vec_to_struct,
    models::Result,
    multisig::MultisigAccountInfo,
    network::NetworkType,
    transaction::{ModifyMultisigAccountTransaction, Transaction},
};

use super::{
    AbstractTransactionDto, CosignatoryModificationDto, TransactionDto, TransactionMetaDto,
};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MultisigDto {
    account: String,
    #[serde(rename = "accountAddress", skip_serializing_if = "Option::is_none")]
    account_address: Option<String>,
    min_approval: i32,
    min_removal: i32,
    cosignatories: Vec<String>,
    multisig_accounts: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MultisigAccountGraphInfoDto {
    #[serde(rename = "level")]
    pub level: i16,
    #[serde(rename = "multisigEntries")]
    multisig_entries: Vec<MultisigAccountInfoDto>,
}

impl MultisigAccountGraphInfoDto {
    pub fn compact(&self) -> crate::Result<Vec<MultisigAccountInfo>> {
        Ok(self
            .multisig_entries
            .iter()
            .map(|item| item.compact().unwrap())
            .collect())
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MultisigAccountInfoDto {
    #[serde(rename = "multisig")]
    multisig: MultisigDto,
}

impl MultisigAccountInfoDto {
    pub fn compact(&self) -> crate::Result<MultisigAccountInfo> {
        let dto = self.multisig.to_owned();
        let network_type: NetworkType =
            Address::from_encoded(&dto.account_address.unwrap())?.network_type;
        let account = PublicAccount::from_public_key(&dto.account, network_type)?;

        let cs = dto
            .cosignatories
            .iter()
            .map(|item| PublicAccount::from_public_key(item, network_type).unwrap())
            .collect();

        let ms = dto
            .multisig_accounts
            .iter()
            .map(|item| PublicAccount::from_public_key(item, network_type).unwrap())
            .collect();

        Ok(MultisigAccountInfo {
            account,
            min_approval: dto.min_approval,
            min_removal: dto.min_removal,
            cosignatories: cs,
            multisig_accounts: ms,
        })
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ModifyMultisigAccountTransactionInfoDto {
    meta: TransactionMetaDto,
    transaction: ModifyMultisigAccountTransactionDto,
}

/// ModifyMultisigAccountTransactionDto : Transaction that creates or modifies a multisig account.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ModifyMultisigAccountTransactionDto {
    #[serde(flatten)]
    r#abstract: AbstractTransactionDto,
    min_removal_delta: i8,
    min_approval_delta: i8,
    modifications: Vec<CosignatoryModificationDto>,
}

#[typetag::serde]
impl TransactionDto for ModifyMultisigAccountTransactionInfoDto {
    fn compact(&self) -> Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();
        let info = self.meta.compact();

        let abs_transaction = dto.r#abstract.compact(info)?;

        let modifications =
            cosignatory_dto_vec_to_struct(dto.modifications, abs_transaction.network_type);

        Ok(Box::new(ModifyMultisigAccountTransaction {
            abs_transaction,
            min_removal_delta: dto.min_removal_delta,
            min_approval_delta: dto.min_approval_delta,
            modifications,
        }))
    }
}
