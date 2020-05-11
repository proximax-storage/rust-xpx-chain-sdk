use sdk::{
    account::Address,
    alias::AliasActionType,
    mosaic::MosaicId,
    namespace::{NamespaceAlias, NamespaceId},
    transaction::{AddressAliasTransaction, AliasTransaction, MosaicAliasTransaction, Transaction},
};

use super::{AbstractTransactionDto, TransactionDto, TransactionMetaDto, Uint64Dto};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AliasDto {
    #[serde(rename = "type")]
    pub _type: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mosaic_id: Option<Uint64Dto>,
    /// The aliased address in hexadecimal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

impl AliasDto {
    pub fn to_struct(&self) -> crate::Result<NamespaceAlias> {
        let mut alias = NamespaceAlias::default();
        alias.type_ = self._type as u8;

        if let Some(a) = &self.address {
            let address = Address::from_encoded(a)?;
            alias.address = Some(address)
        };

        if let Some(m) = &self.mosaic_id {
            let mosaic_id = MosaicId::from(m.to_struct());
            alias.mosaic_id = Some(mosaic_id)
        }

        Ok(alias)
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AliasTransactionDto {
    pub action_type: u8,
    pub namespace_id: Uint64Dto,
}

/// AddressAliasTransactionDto :
/// Transaction that attaches a namespace to an account.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AddressAliasTransactionDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    signature: Option<String>,
    signer: String,
    version: i32,
    #[serde(rename = "type")]
    _type: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_fee: Option<Uint64Dto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deadline: Option<Uint64Dto>,
    namespace_id: Uint64Dto,
    alias_action: u8,
    address: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AddressAliasTransactionInfoDto {
    pub meta: TransactionMetaDto,
    pub transaction: AddressAliasTransactionDto,
}

#[typetag::serde]
impl TransactionDto for AddressAliasTransactionInfoDto {
    fn to_struct(&self) -> crate::Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();

        let info = self.meta.to_struct();

        let address = Address::from_encoded(&dto.address)?;

        let abs = AbstractTransactionDto::new(
            dto.signature.to_owned(),
            dto.signer.to_owned(),
            dto.version,
            dto._type,
            dto.max_fee.to_owned(),
            dto.deadline.to_owned(),
        )
            .to_struct(info)?;

        Ok(Box::new(AddressAliasTransaction {
            alias_transaction: AliasTransaction {
                abs_transaction: abs,
                action_type: AliasActionType::from(dto.alias_action),
                namespace_id: NamespaceId::from(dto.namespace_id.to_struct()),
            },
            address,
        }))
    }
}

/// MosaicAliasTransactionDto :
/// Transaction that attaches a namespace to a mosaic.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicAliasTransactionDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    signature: Option<String>,
    signer: String,
    version: i32,
    #[serde(rename = "type")]
    _type: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_fee: Option<Uint64Dto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deadline: Option<Uint64Dto>,
    alias_action: u8,
    namespace_id: Uint64Dto,
    mosaic_id: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MosaicAliasTransactionInfoDto {
    pub meta: TransactionMetaDto,
    pub transaction: MosaicAliasTransactionDto,
}

#[typetag::serde]
impl TransactionDto for MosaicAliasTransactionInfoDto {
    fn to_struct(&self) -> crate::Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();

        let info = self.meta.to_struct();

        let mosaic_id = MosaicId::from(dto.mosaic_id.to_struct());

        let abs = AbstractTransactionDto::new(
            dto.signature.to_owned(),
            dto.signer.to_owned(),
            dto.version,
            dto._type,
            dto.max_fee.to_owned(),
            dto.deadline.to_owned(),
        )
            .to_struct(info)?;

        Ok(Box::new(MosaicAliasTransaction {
            alias_transaction: AliasTransaction {
                abs_transaction: abs,
                action_type: AliasActionType::from(dto.alias_action),
                namespace_id: NamespaceId::from(dto.namespace_id.to_struct()),
            },
            mosaic_id,
        }))
    }
}
