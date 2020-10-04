use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::{respond_json, respond_ok};
use crate::models::geoloc::{find_today, create, NewGeoloc, UpdateGeoloc, Geoloc};
use crate::validate::validate;
use actix_web::web::{block, Data, HttpResponse, Json, Path};
use rayon::prelude::*;
use serde::Serialize;
use validator::Validate;
use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GeolocResponse {
    pub id: Uuid,
    pub latitude: f64,
    pub longitude: f64,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GeolocsResponse(pub Vec<GeolocResponse>);

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateGeolocRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub user_id: String
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateGeolocRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub user_id: String
}


#[derive(Deserialize)]
pub struct PathByDay {
    user_id: Uuid
}

pub async fn get_geolocs_by_day(path: Path<PathByDay>, pool: Data<PoolType>) -> Result<Json<GeolocsResponse>, ApiError> {
    let geolocs = block(move || find_today(&pool, path.user_id)).await?;
    respond_json(geolocs)
}

pub async fn create_geoloc(
    pool: Data<PoolType>,
    params: Json<CreateGeolocRequest>,
) -> Result<Json<GeolocResponse>, ApiError> {
    validate(&params)?;

    let geoloc_id = Uuid::new_v4();
    let new_geoloc: Geoloc = NewGeoloc{
        id: geoloc_id.to_string(),
        latitude: params.latitude,
        longitude: params.longitude,
        user_id: params.user_id.to_string(),
    }
    .into();
    let geoloc = block(move || create(&pool, &new_geoloc)).await?;
    respond_json(geoloc.into())
}

impl From<Geoloc> for GeolocResponse {
    fn from(geoloc: Geoloc) -> Self {
        GeolocResponse {
            id: Uuid::parse_str(&geoloc.id).unwrap(),
            latitude: geoloc.latitude,
            longitude: geoloc.longitude,
            user_id: Uuid::parse_str(&geoloc.user_id).unwrap(),
            created_at: geoloc.created_at
        }
    }
}

impl From<Vec<Geoloc>> for GeolocsResponse {
    fn from(geolocs: Vec<Geoloc>) -> Self {
        GeolocsResponse(geolocs.into_par_iter().map(|geoloc| geoloc.into()).collect())
    }
}
