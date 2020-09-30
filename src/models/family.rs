use crate::database::PoolType;
use crate::errors::ApiError;
use crate::handlers::family::{FamilyResponse, FamiliesResponse};
use crate::schema::families;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
#[table_name = "families"]
pub struct Family {
    pub id: String,
    pub nom: String,
    pub code: String,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_by: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewFamily{
    pub id: String,
    pub nom: String,
    pub code: String,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "families"]
pub struct UpdateFamily {
    pub id: String,
    pub nom: String,
    pub code: String,
    pub updated_by: String,
}

/// Get all families
pub fn get_all(pool: &PoolType) -> Result<FamiliesResponse, ApiError> {
    use crate::schema::families::dsl::*;
    let conn = pool.get()?;
    let all_families = families.load(&conn)?;

    Ok(all_families.into())
}

pub fn find_by_code(pool: &PoolType, _code: &String) -> Result<FamilyResponse, ApiError> {
    use crate::schema::families::dsl::{code, families};

    let not_found = format!("Family {} not found", _code);
    let conn = pool.get()?;
    let family = families
        .filter(code.eq(_code))
        .first::<Family>(&conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(family.into())
}

/// Find a family by the family's id or error out
pub fn find(pool: &PoolType, family_id: Uuid) -> Result<FamilyResponse, ApiError> {
    use crate::schema::families::dsl::{id, families};

    let not_found = format!("Family {} not found", family_id);
    let conn = pool.get()?;
    let family = families
        .filter(id.eq(family_id.to_string()))
        .first::<Family>(&conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(family.into())
}

/// Create a new family
pub fn create(pool: &PoolType, new_family: &Family) -> Result<FamilyResponse, ApiError> {
    use crate::schema::families::dsl::families;

    let conn = pool.get()?;
    diesel::insert_into(families).values(new_family).execute(&conn)?;
    Ok(new_family.clone().into())
}

/// Update a family
pub fn update(pool: &PoolType, update_family: &UpdateFamily) -> Result<FamilyResponse, ApiError> {
    use crate::schema::families::dsl::{id, families};

    let conn = pool.get()?;
    diesel::update(families)
        .filter(id.eq(update_family.id.clone()))
        .set(update_family)
        .execute(&conn)?;
    find(&pool, Uuid::parse_str(&update_family.id)?)
}

/// Delete a family
pub fn delete(pool: &PoolType, family_id: Uuid) -> Result<(), ApiError> {
    use crate::schema::families::dsl::{id, families};

    let conn = pool.get()?;
    diesel::delete(families)
        .filter(id.eq(family_id.to_string()))
        .execute(&conn)?;
    Ok(())
}

impl From<NewFamily> for Family {
    fn from(family: NewFamily) -> Self {
        Family {
            id: family.id,
            nom: family.nom,
            code: family.code,
            created_by: family.created_by,
            created_at: Utc::now().naive_utc(),
            updated_by: family.updated_by,
            updated_at: Utc::now().naive_utc(),
        }
    }
}