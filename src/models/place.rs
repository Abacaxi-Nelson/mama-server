use crate::database::PoolType;
use crate::errors::ApiError;
use crate::handlers::place::{PlaceResponse, PlacesResponse};
use crate::schema::places;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct Place {
    pub id: String,
    pub name: String,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_by: String,
    pub updated_at: NaiveDateTime,
    pub family_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewPlace{
    pub id: String,
    pub name: String,
    pub family_id: String,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "places"]
pub struct UpdatePlace {
    pub id: String,
    pub name: String,
    pub updated_by: String,
    pub family_id: String,
}

/// Get all places
pub fn get_all(pool: &PoolType) -> Result<PlacesResponse, ApiError> {
    use crate::schema::places::dsl::places;

    let conn = pool.get()?;
    let all_places = places.load(&conn)?;

    Ok(all_places.into())
}

/// Get all places by family_id
pub fn get_all_by_family_id(pool: &PoolType, _family_id: Uuid) -> Result<PlacesResponse, ApiError> {
    use crate::schema::places::dsl::*;

    let conn = pool.get()?;
    let all_places = places
        .filter(family_id.eq(_family_id.to_string()))
        .load(&conn)?;

    Ok(all_places.into())
}

/// Find a place 
pub fn find(pool: &PoolType, place_id: Uuid) -> Result<PlaceResponse, ApiError> {
    use crate::schema::places::dsl::{id, places};

    let not_found = format!("Place {} not found", place_id);
    let conn = pool.get()?;
    let place = places
        .filter(id.eq(place_id.to_string()))
        .first::<Place>(&conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(place.into())
}

/// Create a new place
pub fn create(pool: &PoolType, new_place: &Place) -> Result<PlaceResponse, ApiError> {
    use crate::schema::places::dsl::places;

    let conn = pool.get()?;
    diesel::insert_into(places).values(new_place).execute(&conn)?;
    Ok(new_place.clone().into())
}

/// Update a place
pub fn update(pool: &PoolType, update_place: &UpdatePlace) -> Result<PlaceResponse, ApiError> {
    use crate::schema::places::dsl::{id, places};

    let conn = pool.get()?;
    diesel::update(places)
        .filter(id.eq(update_place.id.clone()))
        .set(update_place)
        .execute(&conn)?;
    find(&pool, Uuid::parse_str(&update_place.id)?)
}

/// Delete a place
pub fn delete(pool: &PoolType, place_id: Uuid) -> Result<(), ApiError> {
    use crate::schema::places::dsl::{id, places};

    let conn = pool.get()?;
    diesel::delete(places)
        .filter(id.eq(place_id.to_string()))
        .execute(&conn)?;
    Ok(())
}

impl From<NewPlace> for Place {
    fn from(place: NewPlace) -> Self {
        Place {
            id: place.id,
            name: place.name,
            created_by: place.created_by,
            created_at: Utc::now().naive_utc(),
            updated_by: place.updated_by,
            updated_at: Utc::now().naive_utc(),
            family_id: place.family_id,
        }
    }
}