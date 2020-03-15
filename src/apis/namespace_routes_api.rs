use ::std::sync::Arc;

use hyper::{client::connect::Connect, Method};

use crate::{
    apis::sirius_client::ApiClient,
    models::{
        errors::ERR_EMPTY_MOSAIC_IDS,
    },
};

use super::{internally::valid_vec_len, request as __internal_request, Result};
use crate::models::namespace::{NamespaceId, NamespaceInfoDto, NamespaceInfo, NamespaceNameDto, NamespaceIds, NamespaceName};
use std::rc::Rc;
use crate::models::errors::ERR_EMPTY_NAMESPACE_IDS;

// Namespace ApiClient routes
const NAMESPACE_ROUTE:    &str           = "/namespace/{namespaceId}";
const NAMESPACES_FROM_ACCOUNTS_ROUTE:    &str = "/account/namespaces";
const NAMESPACE_NAMES_ROUTE:    &str         = "/namespace/names";
const NAMESPACES_FROM_ACCOUNT_ROUTES:    &str = "/account/{accountId}/namespaces";

/// Namespace ApiClient routes.
///
#[derive(Clone)]
pub struct NamespaceRoutes<C: Connect> {
    client: Arc<ApiClient<C>>,
}

/// Namespace related endpoints.
///
impl<C: Connect> NamespaceRoutes<C> where
    C: Clone + Send + Sync + 'static
{
    pub(crate) fn new(client: Arc<ApiClient<C>>) -> Self {
        NamespaceRoutes {
            client,
        }
    }

    pub async fn get_namespace_info(self, namespace_id: NamespaceId) -> Result<NamespaceInfo> {
        let mut req = __internal_request::Request::new(
            Method::GET,
            NAMESPACE_ROUTE.to_string(),
        );

        req = req.with_path_param("namespaceId".to_string(), namespace_id.to_string());

        let dto_raw: Result<NamespaceInfoDto> = req.clone().execute(self.client.to_owned()).await;

        let mut dto_to_struct = dto_raw?.to_struct()?;

        if let Some(parent) = dto_to_struct.parent.to_owned() {
            req = req.with_path_param("namespaceId".to_string(), parent.namespace_id.to_string());

            let dto_raw_parent: Result<NamespaceInfoDto> = req.execute(self.client).await;

            let dto_parent = dto_raw_parent?.to_struct()?;

            dto_to_struct.parent = Some(Box::new(dto_parent));
        }

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

        let dto: Vec<NamespaceNameDto> = req.execute(self.client).await?;

        let mut namespace_name: Vec<NamespaceName> = Vec::with_capacity(dto.len());
        for namespace_name_dto in dto {
            namespace_name.push(namespace_name_dto.to_struct()?);
        }

        Ok(namespace_name)
    }
//
//    /// Get readable names for a set of mosaics
//    ///
//    /// # Inputs
//    ///
//    /// * `mosaics_id` =    The vector of mosaic identifiers.
//    ///
//    /// # Example
//    ///
//    /// ```
//    ///use hyper::Client;
//    ///use xpx_chain_sdk::sirius_client::SiriusClient;
//    ///use xpx_chain_sdk::mosaic::MosaicId;
//    ///
//    ///const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";
//    ///
//    ///#[tokio::main]
//    ///async fn main() {
//    ///
//    /// let client = SiriusClient::new(NODE_URL, Client::new());
//    ///
//    ///    let mosaic_id_a = MosaicId::from_hex("3C520B7CEB2F7099").unwrap();
//    ///    let mosaic_id_b = MosaicId::from_hex("6208AE4D56451357").unwrap();
//    ///
//    ///    let mosaics_names = client.mosaic.get_mosaics_names(vec![mosaic_id_a, mosaic_id_b]).await;
//    ///
//    ///    match mosaics_names {
//    ///        Ok(resp) => {
//    ///            for mosaic in resp {
//    ///                println!("{}", mosaic)
//    ///            }
//    ///        }
//    ///        Err(err) => panic!("{:?}", err),
//    ///    }
//    ///}
//    /// ```
//    ///
//    /// # Returns
//    ///
//    /// Returns a Future `Result` whose okay value is an friendly names for mosaics or
//    /// whose error value is an `Error<Value>` describing the error that occurred.
//    pub async fn get_mosaics_names(self, mosaic_ids: Vec<MosaicId>) -> Result<Vec<MosaicNames>> {
//        valid_vec_len(&mosaic_ids, ERR_EMPTY_MOSAIC_IDS)?;
//
//        let mosaics_ids = MosaicIds::from(mosaic_ids);
//
//        let mut req = __internal_request::Request::new(
//            Method::POST,
//            "/mosaic/names".to_string(),
//        );
//
//        req = req.with_body_param(mosaics_ids);
//
//        let dto: Vec<MosaicNamesDto> = req.execute(self.client).await?;
//
//        let mut mosaics_names: Vec<MosaicNames> = Vec::with_capacity(dto.len());
//        for mosaic_name_dto in dto {
//            mosaics_names.push(mosaic_name_dto.to_struct());
//        }
//
//        Ok(mosaics_names)
//    }
}
