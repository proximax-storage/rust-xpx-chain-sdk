use sdk::{
    account::{Address, PublicAccount},
    multisig::MultisigAccountInfo,
    network::NetworkType,
    transaction::{ModifyMultisigAccountTransaction, Transaction},
};

use crate::internally::cosignatory_dto_vec_to_struct;

use super::{
    AbstractTransactionDto, CosignatoryModificationDto, TransactionDto, TransactionMetaDto,
    Uint64Dto,
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
    pub fn to_struct(&self) -> crate::Result<Vec<MultisigAccountInfo>> {
        Ok(self
            .multisig_entries
            .iter()
            .map(|item| item.to_struct().unwrap())
            .collect())
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MultisigAccountInfoDto {
    #[serde(rename = "multisig")]
    multisig: MultisigDto,
}

impl MultisigAccountInfoDto {
    pub fn to_struct(&self) -> crate::Result<MultisigAccountInfo> {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    signature: Option<String>,
    signer: String,
    version: u32,
    #[serde(rename = "type")]
    _type: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_fee: Option<Uint64Dto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deadline: Option<Uint64Dto>,
    min_removal_delta: i8,
    min_approval_delta: i8,
    modifications: Vec<CosignatoryModificationDto>,
}

#[typetag::serde]
impl TransactionDto for ModifyMultisigAccountTransactionInfoDto {
    fn to_struct(&self) -> crate::Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();
        let info = self.meta.to_struct();

        let abs = AbstractTransactionDto::new(
            dto.signature,
            dto.signer,
            dto.version,
            dto._type,
            dto.max_fee,
            dto.deadline,
        )
            .to_struct(info)?;

        let modifications = cosignatory_dto_vec_to_struct(dto.modifications, abs.network_type);

        Ok(Box::new(ModifyMultisigAccountTransaction {
            abs_transaction: abs,
            min_removal_delta: dto.min_removal_delta,
            min_approval_delta: dto.min_approval_delta,
            modifications,
        }))
    }
}
