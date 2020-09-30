use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::{respond_json, respond_ok};
use crate::models::family::{find_by_code, create, delete, find, get_all, update, NewFamily, UpdateFamily, Family};
use crate::validate::validate;
use actix_web::web::{block, Data, HttpResponse, Json, Path};
use rayon::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;
use rand::Rng;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FamilyResponse {
    pub id: Uuid,
    pub nom: String,
    pub code: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FamiliesResponse(pub Vec<FamilyResponse>);

impl From<Family> for FamilyResponse {
    fn from(family: Family) -> Self {
        FamilyResponse {
            id: Uuid::parse_str(&family.id).unwrap(),
            nom: family.nom.to_string(),
            code: family.code.to_string(),
        }
    }
}

impl From<Vec<Family>> for FamiliesResponse {
    fn from(families: Vec<Family>) -> Self {
        FamiliesResponse(families.into_par_iter().map(|family| family.into()).collect())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateFamilyRequest {
    #[validate(length(
        min = 3,
        message = "nom is required and must be at least 3 characters"
    ))]
    pub nom: String,
}

/// Get a family
pub async fn get_family(
    family_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<Json<FamilyResponse>, ApiError> {
    let family = block(move || find(&pool, *family_id)).await?;
    respond_json(family)
}

#[derive(Deserialize)]
pub struct PathByCode {
    code: String
}

pub async fn get_family_by_code(
    path: Path<PathByCode>,
    pool: Data<PoolType>,
) -> Result<Json<FamilyResponse>, ApiError> {
    println!("get_family_by_code");
    println!("{:?}", &path.code);
    let family = block(move || find_by_code(&pool, &path.code)).await?;
    println!("get_family_by_code 2");
    respond_json(family)
}

/// Get all families
pub async fn get_families(pool: Data<PoolType>) -> Result<Json<FamiliesResponse>, ApiError> {
    let families = block(move || get_all(&pool)).await?;
    respond_json(families)
}

/// Create a family
pub async fn create_family(
    pool: Data<PoolType>,
    params: Json<CreateFamilyRequest>,
) -> Result<Json<FamilyResponse>, ApiError> {
    validate(&params)?;

    let mut rng = rand::thread_rng();
    let number: u32 = rng.gen_range(0, 999999);
    let code = format!("{:06}", number);

    let family_id = Uuid::new_v4();
    let new_family: Family = NewFamily{
        id: family_id.to_string(),
        nom: params.nom.to_string(),
        code: code,
        created_by: family_id.to_string(),
        updated_by: family_id.to_string(),
    }
    .into();
    let family = block(move || create(&pool, &new_family)).await?;
    respond_json(family.into())
}


#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateFamilyRequest {
    #[validate(length(
        min = 3,
        message = "nom is required and must be at least 3 characters"
    ))]
    pub nom: String,

    #[validate(length(
        min = 3,
        message = "code is required and must be at least 3 characters"
    ))]
    pub code: String,
}

/// Update a family
pub async fn update_family(
    family_id: Path<Uuid>,
    pool: Data<PoolType>,
    params: Json<UpdateFamilyRequest>,
) -> Result<Json<FamilyResponse>, ApiError> {
    validate(&params)?;

    let update_family= UpdateFamily {
        id: family_id.to_string(),
        nom: params.nom.to_string(),
        code: params.code.to_string(),
        updated_by: family_id.to_string(),
    };
    let family = block(move || update(&pool, &update_family)).await?;
    respond_json(family.into())
}

/// Delete a family
pub async fn delete_family(
    family_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<HttpResponse, ApiError> {
    block(move || delete(&pool, *family_id)).await?;
    respond_ok()
}