use ::std::sync::Arc;

use hyper::{client::connect::Connect, Method};

use crate::{
    apis::sirius_client::ApiClient,
    models::{
        errors::ERR_EMPTY_MOSAIC_IDS,
    },
};

use super::{internally::valid_vec_len, request as __internal_request, Result};
use crate::models::namespace::{NamespaceId, NamespaceInfoDto, NamespaceInfo};
use std::rc::Rc;

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

    /// Get [Mosaic] information.
    ///
    /// Gets the mosaic definition for a given mosaic_id.
    ///
    /// # Inputs
    ///
    /// * `mosaic_id` =    The mosaic identifier.
    ///
    /// # Example
    ///
    /// ```
    ///use hyper::Client;
    ///use xpx_chain_sdk::sirius_client::SiriusClient;
    ///use xpx_chain_sdk::mosaic::MosaicId;
    ///
    ///const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///
    /// let client = SiriusClient::new(NODE_URL, Client::new());
    ///
    ///    let mosaic_id = MosaicId::from_hex("3C520B7CEB2F7099").unwrap();
    ///
    ///    let mosaic_info = client.mosaic.namespace_info(mosaic_id).await;
    ///
    ///    match mosaic_info {
    ///        Ok(resp_info) => println!("{}", resp_info),
    ///        Err(err) => eprintln!("{:?}", err),
    ///    }
    ///}
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Future `Result` whose okay value is an [MosaicInfo] or
    /// whose error value is an `Error<Value>` describing the error that occurred.
    pub async fn namespace_info(self, namespace_id: NamespaceId) -> Result<NamespaceInfo> {
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
//
//    /// Gets an vector of [MosaicInfo] definition.
//    ///
//    /// Get mosaics information for an vector of mosaics
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
//    ///    let mosaics_info = client.mosaic.get_mosaics_info(vec![mosaic_id_a, mosaic_id_b]).await;
//    ///
//    ///    match mosaics_info {
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
//    /// Returns a Future `Result` whose okay value is an verctor of [MosaicInfo] or
//    /// whose error value is an `Error<Value>` describing the error that occurred.
//    pub async fn get_mosaics_info(self, mosaic_ids: Vec<MosaicId>) -> Result<Vec<MosaicInfo>> {
//        valid_vec_len(&mosaic_ids, ERR_EMPTY_MOSAIC_IDS)?;
//
//        let mosaics_ids = MosaicIds::from(mosaic_ids);
//        let mut req = __internal_request::Request::new(
//            Method::POST,
//            "/mosaic".to_string(),
//        );
//
//        req = req.with_body_param(mosaics_ids);
//
//        let dto: Vec<MosaicInfoDto> = req.execute(self.client).await?;
//
//        let mut mosaics_info: Vec<MosaicInfo> = Vec::with_capacity(dto.len());
//        for mosaic_info_dto in dto {
//            mosaics_info.push(mosaic_info_dto.to_struct()?);
//        }
//
//        Ok(mosaics_info)
//    }
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
