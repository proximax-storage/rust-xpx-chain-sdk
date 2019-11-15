#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionInfoDto {
    #[serde(rename = "meta")]
    pub meta: crate::models::TransactionMetaDto,
    #[serde(rename = "transaction")]
    pub transaction: crate::models::AnyOfBlockchainUpgradeTransactionDtoNetworkConfigTransactionDtoAddressMetadataTransactionDtoMosaicMetadataTransactionDtoNamespaceMetadataTransactionDtoMosaicDefinitionTransactionDtoMosaicSupplyChangeTransactionDtoRegisterNamespaceTransactionDtoAddressAliasTransactionDtoMosaicAliasTransactionDtoTransferTransactionDtoModifyMultisigAccountTransactionDtoAggregateTransactionDtoHashLockTransactionDtoAccountPropertiesTransactionDtoSecretLockTransactionDtoSecretProofTransactionDtoAccountLinkTransactionDto,
}

impl TransactionInfoDto {
    pub fn new(meta: crate::models::TransactionMetaDto, transaction: crate::models::AnyOfBlockchainUpgradeTransactionDtoNetworkConfigTransactionDtoAddressMetadataTransactionDtoMosaicMetadataTransactionDtoNamespaceMetadataTransactionDtoMosaicDefinitionTransactionDtoMosaicSupplyChangeTransactionDtoRegisterNamespaceTransactionDtoAddressAliasTransactionDtoMosaicAliasTransactionDtoTransferTransactionDtoModifyMultisigAccountTransactionDtoAggregateTransactionDtoHashLockTransactionDtoAccountPropertiesTransactionDtoSecretLockTransactionDtoSecretProofTransactionDtoAccountLinkTransactionDto) -> TransactionInfoDto {
        TransactionInfoDto {
            meta,
            transaction,
        }
    }
}


