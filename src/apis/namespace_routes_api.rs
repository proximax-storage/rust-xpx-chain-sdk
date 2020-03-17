use ::std::sync::Arc;
use std::future::Future;
use std::pin::Pin;

use hyper::{client::connect::Connect, Method};

use crate::apis::sirius_client::ApiClient;
use crate::models::account::Address;
use crate::models::errors::ERR_EMPTY_NAMESPACE_IDS;
use crate::models::id_model::Id;
use crate::models::namespace::{NamespaceId, NamespaceIds, NamespaceInfo, NamespaceInfoDto,
                               NamespaceName, NamespaceNameDto
};
use crate::Uint64;

use super::{internally::valid_vec_len, request as __internal_request, Result};

const NAMESPACE_ROUTE: &str = "/namespace/{namespaceId}";
const NAMESPACES_FROM_ACCOUNTS_ROUTE: &str = "/account/namespaces";
const NAMESPACE_NAMES_ROUTE: &str = "/namespace/names";
const NAMESPACES_FROM_ACCOUNT_ROUTES: &str = "/account/{accountId}/namespaces";

/// Namespace ApiClient routes.
///
#[derive(Clone)]
pub struct NamespaceRoutes<C: Connect> (Arc<ApiClient<C>>);

/// Namespace related endpoints.
///
impl<C: Connect> NamespaceRoutes<C> where
    C: Clone + Send + Sync + 'static
{
    pub(crate) fn new(client: Arc<ApiClient<C>>) -> Self {
        NamespaceRoutes(client)
    }

    fn __build_namespace_hierarchy<'b>(self, ns_info: &'b mut NamespaceInfo) -> Pin<Box<dyn Future<Output = ()> + 'b>> {
        Box::pin(async move {
            let info_parent = match &ns_info.parent {
                Some(info) => info,
                _ => return ()
            };

            if info_parent.namespace_id.to_u64() == 0 {
                return ()
            }

            let rest_info = self.clone().get_namespace_info(info_parent.namespace_id).await;
            let mut parent_ns_info = match rest_info {
                Ok(parent) => Box::new(parent),
                _ => return ()
            };

            ns_info.parent = Some(parent_ns_info.to_owned());

            if let None = parent_ns_info.to_owned().parent {
                return ()
            }

            self.__build_namespace_hierarchy(&mut parent_ns_info).await
        })
    }

    fn __build_namespaces_hierarchy<'b>(self, ns_infos: &'b mut Vec<NamespaceInfo>) -> Pin<Box<dyn Future<Output = ()> + 'b>> {
        Box::pin(async move {
            for ns_info in ns_infos.iter_mut() {
                self.clone().__build_namespace_hierarchy(ns_info).await
            }
        })
    }

    pub async fn get_namespace_info(self, namespace_id: NamespaceId) -> Result<NamespaceInfo> {
        let mut req = __internal_request::Request::new(
            Method::GET,
            NAMESPACE_ROUTE.to_string(),
        );

        req = req.with_path_param("namespaceId".to_string(), namespace_id.to_string());

        let dto_raw: Result<NamespaceInfoDto> = req.clone().execute(self.0.to_owned()).await;

        let mut dto_to_struct = dto_raw?.to_struct()?;

        self.__build_namespace_hierarchy(&mut dto_to_struct).await;

        Ok(dto_to_struct)
    }

    pub async fn get_namespaces_names(self, namespace_ids: Vec<NamespaceId>) -> Result<Vec<NamespaceName>> {
        valid_vec_len(&namespace_ids, ERR_EMPTY_NAMESPACE_IDS)?;

        let namespace_ids_ = NamespaceIds::from(namespace_ids);
        let mut req = __internal_request::Request::new(
            Method::POST,
            NAMESPACE_NAMES_ROUTE.to_string(),
        );

        req = req.with_body_param(namespace_ids_);

        let dto: Vec<NamespaceNameDto> = req.execute(self.0).await?;

        let mut namespace_name: Vec<NamespaceName> = Vec::with_capacity(dto.len());
        for namespace_name_dto in dto {
            namespace_name.push(namespace_name_dto.to_struct()?);
        }

        Ok(namespace_name)
    }

    pub async fn get_namespaces_from_account(self, address: Address, ns_id: Option<NamespaceId>, page_size: Option<i32>) -> Result<Vec<NamespaceInfo>>
    {
        let mut req = __internal_request::Request::new(
            Method::GET,
            NAMESPACES_FROM_ACCOUNT_ROUTES.to_string(),
        );

        if let Some(ref s) = page_size {
            req = req.with_query_param("pageSize".to_string(), s.to_string());
        }
        if let Some(ref s) = ns_id {
            req = req.with_query_param("id".to_string(), s.to_hex());
        }

        req = req.with_path_param("accountId".to_string(), address.address.to_string());

        let dto: Vec<NamespaceInfoDto> = req.execute(self.0.to_owned()).await?;

        let mut namespace_info: Vec<NamespaceInfo> = Vec::with_capacity(dto.len());
        for namespace_dto in dto {
            namespace_info.push(namespace_dto.to_struct()?);
        }

        self.__build_namespaces_hierarchy(&mut namespace_info).await;

        Ok(namespace_info)
    }
}
