/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::{
    api::{
        error::{Error, Result},
        valid_vec_len,
    },
    errors_const::{ERR_INVALID_ASSET_ID, ERR_INVALID_NAMESPACE_ALIASED},
    mosaic::{AssetIdType, MosaicInfo, UnresolvedMosaicId},
};
use crate::api::transport::service::Connection;

use super::{mosaic_routes_api::MosaicRoutes, namespace_routes_api::NamespaceRoutes};

/// Node ApiClient routes.
///
#[derive(Clone)]
pub struct ResolverRoutes(Connection, NamespaceRoutes, MosaicRoutes);

/// Resolver related endpoints.
///
impl ResolverRoutes {
    pub(crate) fn new(
        client: Connection,
        namespace_routes: NamespaceRoutes,
        mosaic_routes: MosaicRoutes,
    ) -> Self {
        ResolverRoutes(client, namespace_routes, mosaic_routes)
    }

    fn __client(&self) -> Connection {
        self.0.clone()
    }

    fn __namespace_routes(&self) -> NamespaceRoutes {
        self.1.clone()
    }

    fn __mosaic_routes(&self) -> MosaicRoutes {
        self.2.clone()
    }

    pub async fn get_mosaic_info_by_asset_id(
        &self,
        asset_id: Box<dyn UnresolvedMosaicId>,
    ) -> Result<MosaicInfo> {
        match asset_id.get_type() {
            AssetIdType::NamespaceIdType => {
                let namespace_id = asset_id.as_namespace_id()?;

                let namespace_info =
                    self.clone().__namespace_routes().get_namespace_info(namespace_id).await?;

                if namespace_info.alias.mosaic_id.is_none() {
                    return Err(Error::from(ERR_INVALID_NAMESPACE_ALIASED));
                };

                self.__mosaic_routes()
                    .get_mosaic_info(namespace_info.alias.mosaic_id.unwrap())
                    .await
            }
            AssetIdType::MosaicIdType => {
                let mosaic_id = asset_id.as_mosaic_id()?;
                self.__mosaic_routes().get_mosaic_info(mosaic_id).await
            }
        }
    }

    pub async fn get_mosaic_info_by_asset_ids(
        &self,
        asset_ids: Vec<Box<dyn UnresolvedMosaicId>>,
    ) -> Result<Vec<MosaicInfo>> {
        valid_vec_len(&asset_ids, ERR_INVALID_ASSET_ID)?;

        let mut namespace_ids = vec![];
        let mut mosaic_ids = vec![];

        for asset_id in &asset_ids {
            match asset_id.get_type() {
                AssetIdType::NamespaceIdType => {
                    let namespace_id = asset_id.as_namespace_id()?;
                    namespace_ids.push(namespace_id);
                }
                AssetIdType::MosaicIdType => {
                    let mosaic_id = asset_id.as_mosaic_id()?;
                    mosaic_ids.push(mosaic_id);
                }
            }
        }

        if !mosaic_ids.is_empty() {
            self.__mosaic_routes().get_mosaics_info(mosaic_ids).await
        } else {
            let mut mosaic_infos = vec![];

            for namespace_id in namespace_ids {
                mosaic_infos
                    .push(self.clone().get_mosaic_info_by_asset_id(Box::new(namespace_id)).await?);
            }
            Ok(mosaic_infos)
        }
    }
}
