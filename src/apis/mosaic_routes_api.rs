use ::std::fmt::Debug;
use ::std::sync::Arc;

use hyper::{client::connect::Connect, Method};

use crate::{
    apis::sirius_client::ApiClient,
    models::{
        errors::ERR_EMPTY_MOSAIC_IDS,
        mosaic::{MosaicId, MosaicIds, MosaicInfo, MosaicInfoDto, MosaicNames, MosaicNamesDto}
    },
};

use super::{internally::valid_vec_len, request as __internal_request, Result};

#[derive(Clone)]
pub struct MosaicRoutes<C: Connect> {
    client: Arc<ApiClient<C>>,
}

impl<C: Connect> MosaicRoutes<C> {
    pub fn new(client: Arc<ApiClient<C>>) -> Self {
        MosaicRoutes {
            client,
        }
    }
}

impl<C: Connect> MosaicRoutes<C> where
    C: Clone + Send + Sync + Debug + 'static
{
    pub async fn get_mosaic_info(self, mosaic_id: MosaicId) -> Result<MosaicInfo> {
        let mut req = __internal_request::Request::new(
            Method::GET,
            "/mosaic/{mosaic_id}".to_string(),
        );

        req = req.with_path_param("mosaic_id".to_string(), mosaic_id.to_string());

        let dto: Result<MosaicInfoDto> = req.execute(self.client).await;

        Ok(dto?.to_struct()?)
    }

    pub async fn get_mosaics_info(self, mosaic_ids: Vec<MosaicId>) -> Result<Vec<MosaicInfo>> {
        valid_vec_len(&mosaic_ids, ERR_EMPTY_MOSAIC_IDS)?;

        let mosaics_ids = MosaicIds::from(mosaic_ids);
        let mut req = __internal_request::Request::new(
            Method::POST,
            "/mosaic".to_string(),
        );

        req = req.with_body_param(mosaics_ids);

        let dto: Vec<MosaicInfoDto> = req.execute(self.client).await?;

        let mut mosaics_info: Vec<MosaicInfo> = Vec::with_capacity(dto.len());
        for i in dto {
            let mosaic_info = i;
            mosaics_info.push(mosaic_info.to_struct()?);
        }

        Ok(mosaics_info)
    }

    pub async fn get_mosaics_names(self, mosaic_ids: Vec<MosaicId>) -> Result<Vec<MosaicNames>> {
        valid_vec_len(&mosaic_ids, ERR_EMPTY_MOSAIC_IDS)?;

        let mosaics_ids = MosaicIds::from(mosaic_ids);

        let mut req = __internal_request::Request::new(
            Method::POST,
            "/mosaic/names".to_string(),
        );

        req = req.with_body_param(mosaics_ids);

        let dto: Vec<MosaicNamesDto> = req.execute(self.client).await?;

        let mut mosaics_names: Vec<MosaicNames> = Vec::with_capacity(dto.len());
        for i in dto {
            let mosaic_name = i;
            mosaics_names.push(mosaic_name.to_struct());
        }

        Ok(mosaics_names)
    }
}
