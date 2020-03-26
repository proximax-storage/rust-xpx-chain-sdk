use crate::models::multisig::CosignatoryModificationDto;
use crate::models::uint_64::Uint64Dto;
use crate::models::transaction::{TransactionMetaDto, ModifyMultisigAccountTransaction, AbstractTransactionDto, TransactionDto, Transaction, cosignatory_dto_vec_to_struct};

#[derive(Serialize, Deserialize)]
pub(crate) struct MultisigDto {
    /// The account public key.
    #[serde(rename = "account")]
    pub account: String,
    /// The account address in hexadecimal.
    #[serde(rename = "accountAddress", skip_serializing_if = "Option::is_none")]
    pub account_address: Option<String>,
    /// The number of signatures needed to approve a transaction.
    #[serde(rename = "minApproval")]
    pub min_approval: i32,
    /// The number of signatures needed to remove a cosignatory.
    #[serde(rename = "minRemoval")]
    pub min_removal: i32,
    /// The array of public keys of the cosignatory accounts.
    #[serde(rename = "cosignatories")]
    pub cosignatories: Vec<String>,
    /// The array of multisig accounts where the account is cosignatory.
    #[serde(rename = "multisigAccounts")]
    pub multisig_accounts: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MultisigAccountGraphInfoDto {
    /// The level of the multisig account.
    #[serde(rename = "level")]
    pub level: i32,
    /// The array of multisig accounts for this level.
    #[serde(rename = "multisigEntries")]
    pub multisig_entries: Vec<MultisigAccountInfoDto>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MultisigAccountInfoDto {
    #[serde(rename = "multisig")]
    pub multisig: MultisigDto,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ModifyMultisigAccountTransactionInfoDto {
    pub meta: TransactionMetaDto,
    pub transaction: ModifyMultisigAccountTransactionDto,
}

/// ModifyMultisigAccountTransactionDto : Transaction that creates or modifies a multisig account.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ModifyMultisigAccountTransactionDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    pub signer: String,
    pub version: u32,
    #[serde(rename = "type")]
    pub _type: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<Uint64Dto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Uint64Dto>,
    pub min_removal_delta: i8,
    pub min_approval_delta: i8,
    pub modifications: Vec<CosignatoryModificationDto>,
}

#[typetag::serde]
impl TransactionDto for ModifyMultisigAccountTransactionInfoDto {
    fn to_struct(&self) -> crate::Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();
        let info = self.meta.to_struct();

        let abs = AbstractTransactionDto::new(
            dto.signature, dto.signer, dto.version, dto._type, dto.max_fee, dto.deadline,
        ).to_struct(info)?;

        let modifications = cosignatory_dto_vec_to_struct(dto.modifications, abs.network_type);

        Ok(Box::new(ModifyMultisigAccountTransaction{
            abs_transaction: abs,
            min_removal_delta: dto.min_removal_delta,
            min_approval_delta: dto.min_approval_delta,
            modifications
        }))
    }
}