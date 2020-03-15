use crate::models::field_dto::FieldDto;
use crate::models::metadata_dto::{MetadataModificationDto, MetadataTypeEnum};
use super::{NamespaceId, NamespaceType};
use crate::models::transaction::{
    AbstractTransactionDto,
    RegisterNamespaceTransaction,
    Transaction, TransactionDto,
    TransactionMetaDto
};
use crate::models::uint_64::Uint64Dto;

#[derive(Serialize, Deserialize)]
pub(crate) struct NamespaceDto {
    /// The public key of the owner of the namespace.
    #[serde(rename = "owner")]
    pub owner: String,
    /// The address of the owner of the namespace in hexadecimal.
    #[serde(rename = "ownerAddress")]
    pub owner_address: String,
    #[serde(rename = "startHeight")]
    pub start_height: Uint64Dto,
    #[serde(rename = "endHeight")]
    pub end_height: Uint64Dto,
    /// The level of the namespace.
    #[serde(rename = "depth")]
    pub depth: i32,
    #[serde(rename = "level0")]
    pub level0: Uint64Dto,
    #[serde(rename = "level1", skip_serializing_if = "Option::is_none")]
    pub level1: Option<Uint64Dto>,
    #[serde(rename = "level2", skip_serializing_if = "Option::is_none")]
    pub level2: Option<Uint64Dto>,
    #[serde(rename = "type")]
    pub _type: u8,
    #[serde(rename = "alias")]
    pub alias: crate::models::alias::AliasDto,
    #[serde(rename = "parentId")]
    pub parent_id: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NamespaceIds {
    /// The array of namespace identifiers.
    #[serde(rename = "namespaceIds", skip_serializing_if = "Option::is_none")]
    pub namespace_ids: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NamespaceInfoDto {
    #[serde(rename = "meta")]
    pub meta: crate::models::namespace::NamespaceMetaDto,
    #[serde(rename = "namespace")]
    pub namespace: crate::models::namespace::NamespaceDto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NamespaceMetaDto {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "index")]
    pub index: i32,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NamespaceMetadataDto {
    #[serde(rename = "metadataType")]
    pub metadata_type: i32,
    #[serde(rename = "fields")]
    pub fields: Vec<FieldDto>,
    #[serde(rename = "metadataId")]
    pub metadata_id: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NamespaceMetadataDtoAllOf {
    #[serde(rename = "metadataType", skip_serializing_if = "Option::is_none")]
    pub metadata_type: Option<i32>,
    #[serde(rename = "metadataId")]
    pub metadata_id: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NamespaceMetadataInfoDto {
    #[serde(rename = "metadata")]
    pub metadata: NamespaceMetadataDto,
}

/// NamespaceMetadataTransactionDto : Transaction that addes metadata to namespace.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NamespaceMetadataTransactionDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    pub signer: String,
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<Uint64Dto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Uint64Dto>,
    pub metadata_id: Uint64Dto,
    pub metadata_type: MetadataTypeEnum,
    pub modifications: Vec<MetadataModificationDto>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NamespaceNameDto {
    #[serde(rename = "namespace_id")]
    pub namespace_id: Uint64Dto,
    /// The full name of the namespace.
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RegisterNamespaceTransactionInfoDto {
    pub meta: TransactionMetaDto,
    pub transaction: RegisterNamespaceTransactionDto,
}

/// RegisterNamespaceTransactionDto : Transaction that creates or renew a namespace.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RegisterNamespaceTransactionDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    pub signer: String,
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<Uint64Dto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Uint64Dto>,
    #[serde(rename = "namespaceType")]
    pub namespace_type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Uint64Dto>,
    pub namespace_id: Uint64Dto,
    #[serde(rename = "name")]
    pub namespace_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Uint64Dto>
}

#[typetag::serde]
impl TransactionDto for RegisterNamespaceTransactionInfoDto {
    fn to_struct(&self) -> crate::Result<Box<dyn Transaction>> {
        let dto = self.transaction.clone();

        let info = self.meta.to_struct();

        let abs_transaction = AbstractTransactionDto::new(
            dto.signature, dto.signer, dto.version, dto._type, dto.max_fee, dto.deadline,
        ).to_struct(info)?;

        let namespace_type = NamespaceType::from(dto.namespace_type);

        let namespace_id = NamespaceId::from(dto.namespace_id.to_struct());

        let mut parent_id = None;

        let mut duration = None;
        if namespace_type == NamespaceType::Root {
            if let Some(d) = dto.duration {
                duration = Some(d.to_struct())
            };
        } else {
            if let Some(p) = dto.parent_id {
                parent_id = Some(NamespaceId::from(p.to_struct()))
            };
        }

        Ok(Box::new(RegisterNamespaceTransaction {
            abs_transaction,
            namespace_type,
            namespace_id,
            name: dto.namespace_name,
            duration,
            parent_id
        }))
    }
}
