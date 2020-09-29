use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::{respond_json, respond_ok};
use crate::models::place::{create, delete, find, get_all_by_family_id, get_all, update, NewPlace, UpdatePlace, Place};
use crate::validate::validate;
use actix_web::web::{block, Data, HttpResponse, Json, Path};
use rayon::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PlaceResponse {
    pub id: Uuid,
    pub name: String,
    pub family_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PlacesResponse(pub Vec<PlaceResponse>);

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreatePlaceRequest {
    #[validate(length(
        min = 3,
        message = "name is required and must be at least 3 characters"
    ))]
    pub name: String,

    #[validate(length(
        min = 3,
        message = "family_id is required and must be at least 3 characters"
    ))]
    pub family_id: String
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdatePlaceRequest {
    #[validate(length(
        min = 3,
        message = "name is required and must be at least 3 characters"
    ))]
    pub name: String,

    #[validate(length(
        min = 3,
        message = "family_id is required and must be at least 3 characters"
    ))]
    pub family_id: String
}

pub async fn get_place(
    place_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<Json<PlaceResponse>, ApiError> {
    let place = block(move || find(&pool, *place_id)).await?;
    respond_json(place)
}

pub async fn get_places(pool: Data<PoolType>) -> Result<Json<PlacesResponse>, ApiError> {
    let places = block(move || get_all(&pool)).await?;
    respond_json(places)
}

#[derive(Deserialize)]
pub struct PathByFamilyID {
    family_id: Uuid
}

pub async fn get_places_by_family_id(path: Path<PathByFamilyID>, pool: Data<PoolType>) -> Result<Json<PlacesResponse>, ApiError> {
    let places = block(move || get_all_by_family_id(&pool, path.family_id)).await?;
    respond_json(places)
}

pub async fn create_place(
    pool: Data<PoolType>,
    params: Json<CreatePlaceRequest>,
) -> Result<Json<PlaceResponse>, ApiError> {
    validate(&params)?;

    let place_id = Uuid::new_v4();
    let new_place: Place = NewPlace{
        id: place_id.to_string(),
        name: params.name.to_string(),
        family_id: params.family_id.to_string(),
        created_by: place_id.to_string(),
        updated_by: place_id.to_string(),
    }
    .into();
    let place = block(move || create(&pool, &new_place)).await?;
    respond_json(place.into())
}

pub async fn update_place(
    place_id: Path<Uuid>,
    pool: Data<PoolType>,
    params: Json<UpdatePlaceRequest>,
) -> Result<Json<PlaceResponse>, ApiError> {
    validate(&params)?;

    let update_place = UpdatePlace {
        id: place_id.to_string(),
        name: params.name.to_string(),
        family_id: params.family_id.to_string(),
        updated_by: place_id.to_string(),
    };
    let place = block(move || update(&pool, &update_place)).await?;
    respond_json(place.into())
}

pub async fn delete_place(
    place_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<HttpResponse, ApiError> {
    block(move || delete(&pool, *place_id)).await?;
    respond_ok()
}

impl From<Place> for PlaceResponse {
    fn from(place: Place) -> Self {
        PlaceResponse {
            id: Uuid::parse_str(&place.id).unwrap(),
            name: place.name.to_string(),
            family_id: Uuid::parse_str(&place.family_id).unwrap(),
        }
    }
}

impl From<Vec<Place>> for PlacesResponse {
    fn from(places: Vec<Place>) -> Self {
        PlacesResponse(places.into_par_iter().map(|place| place.into()).collect())
    }
}
