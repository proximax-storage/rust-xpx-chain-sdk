/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    ::std::{future::Future, pin::Pin},
    hyper::Method,
};

use crate::{
    account::{AccountsId, Address},
    api::{
        error::Result, internally::valid_vec_len, NamespaceInfoDto, NamespaceNameDto,
        request as __internal_request,
    },
    errors_const::{ERR_EMPTY_ADDRESSES_IDS, ERR_EMPTY_NAMESPACE_IDS},
    mosaic::UnresolvedMosaicId,
    namespace::{NamespaceId, NamespaceIds, NamespaceInfo, NamespaceName},
};
use crate::api::{QueryParam, QueryParams};
use crate::api::error::Error;
use crate::api::transport::service::Connection;

use super::{
    NAMESPACE_NAMES_ROUTE, NAMESPACE_ROUTE, NAMESPACES_FROM_ACCOUNT_ROUTES,
    NAMESPACES_FROM_ACCOUNTS_ROUTE,
};

/// Namespace ApiClient routes.
///
#[derive(Clone)]
pub struct NamespaceRoutes(Connection);

/// Namespace related endpoints.
///
impl NamespaceRoutes {
    pub(crate) fn new(client: Connection) -> Self {
        NamespaceRoutes(client)
    }

    fn __client(&self) -> Connection {
        self.0.clone()
    }

    fn __build_namespace_hierarchy<'a>(
        &'a self,
        ns_info: &'a mut NamespaceInfo,
    ) -> Pin<Box<dyn Future<Output=()> + '_>> {
        Box::pin(async move {
            let info_parent = match &ns_info.parent {
                Some(info) => info,
                _ => return,
            };

            if info_parent.namespace_id.to_u64() == 0 {
                return;
            }

            let rest_info = self.get_namespace_info(info_parent.namespace_id).await;
            let mut parent_ns_info = match rest_info {
                Ok(parent) => Box::new(parent),
                _ => return,
            };

            ns_info.parent = Some(parent_ns_info.to_owned());

            if parent_ns_info.to_owned().parent.is_none() {
                return;
            }

            self.__build_namespace_hierarchy(&mut parent_ns_info).await
        })
    }

    fn __build_namespaces_hierarchy<'b>(
        &'b self,
        ns_infos: &'b mut Vec<NamespaceInfo>,
    ) -> Pin<Box<dyn Future<Output=()> + 'b>> {
        Box::pin(async move {
            for ns_info in ns_infos.iter_mut() {
                self.__build_namespace_hierarchy(ns_info).await
            }
        })
    }

    pub async fn get_namespace_info(&self, namespace_id: NamespaceId) -> Result<NamespaceInfo> {
        let mut req = __internal_request::ApiRequest::new(Method::GET, NAMESPACE_ROUTE.to_string());

        req = req.with_path_param("namespaceId".to_string(), namespace_id.to_string());

        let dto_raw: Result<NamespaceInfoDto> = req.clone().execute(self.__client()).await;

        let mut dto_to_struct = dto_raw?.compact()?;

        self.__build_namespace_hierarchy(&mut dto_to_struct).await;

        Ok(dto_to_struct)
    }

    pub async fn get_namespaces_names(
        &self,
        namespace_ids: Vec<NamespaceId>,
    ) -> Result<Vec<NamespaceName>> {
        valid_vec_len(&namespace_ids, ERR_EMPTY_NAMESPACE_IDS)?;

        let namespace_ids_ = NamespaceIds::from(namespace_ids);
        let mut req =
            __internal_request::ApiRequest::new(Method::POST, NAMESPACE_NAMES_ROUTE.to_string());

        req = req.with_body_param(namespace_ids_);

        let dto: Vec<NamespaceNameDto> = req.execute(self.__client()).await?;

        let mut namespace_name: Vec<NamespaceName> = vec![];
        for namespace_name_dto in dto.into_iter() {
            namespace_name.push(namespace_name_dto.compact()?);
        }

        Ok(namespace_name)
    }

    /// Get namespaces owned by an account
    ///
    /// Gets an vector of `NamespaceInfo` for a given account address.
    pub async fn get_namespaces_from_account(
        &self,
        address: Address,
        query_params: Option<QueryParams>,
    ) -> Result<Vec<NamespaceInfo>> {
        let mut req = __internal_request::ApiRequest::new(
            Method::GET,
            NAMESPACES_FROM_ACCOUNT_ROUTES.to_string(),
        );

        let mut query_params_vec: Vec<QueryParam> = vec![];

        if let Some(query_params) = query_params {
            query_params_vec.append(&mut query_params.to_query_params());
        }

        req = req.with_path_param("accountId".to_string(), address.address_str());

        let dto: Vec<NamespaceInfoDto> = req.execute(self.__client()).await?;

        let mut namespace_info: Vec<NamespaceInfo> = vec![];
        for namespace_dto in dto.into_iter() {
            namespace_info.push(namespace_dto.compact()?);
        }

        self.clone().__build_namespaces_hierarchy(&mut namespace_info).await;

        Ok(namespace_info)
    }

    /// Gets namespaces for a given List of addresses.
    ///
    pub async fn get_namespaces_from_accounts(
        &self,
        accounts_id: Vec<Address>,
        query_params: Option<QueryParams>,
    ) -> Result<Vec<NamespaceInfo>> {
        if accounts_id.is_empty() {
            return Err(Error::from(ERR_EMPTY_ADDRESSES_IDS));
        }

        let accounts = AccountsId::from(
            accounts_id.into_iter().map(|item| item.address_str()).collect::<Vec<_>>(),
        );

        let mut req = __internal_request::ApiRequest::new(
            Method::POST,
            NAMESPACES_FROM_ACCOUNTS_ROUTE.to_string(),
        );

        let mut query_params_vec: Vec<QueryParam> = vec![];

        if let Some(query_params) = query_params {
            query_params_vec.append(&mut query_params.to_query_params());
        }

        req = req.with_body_param(&accounts);

        let dto: Vec<NamespaceInfoDto> = req.execute(self.__client()).await?;

        let mut namespace_info: Vec<NamespaceInfo> = vec![];
        for namespace_dto in dto.into_iter() {
            namespace_info.push(namespace_dto.compact()?);
        }

        self.__build_namespaces_hierarchy(&mut namespace_info).await;

        Ok(namespace_info)
    }
}
