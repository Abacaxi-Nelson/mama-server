use crate::database::PoolType;
use crate::errors::ApiError;
use crate::handlers::geoloc::{GeolocResponse, GeolocsResponse};
use crate::schema::geolocs;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
#[table_name = "geolocs"]
pub struct Geoloc {
    pub id: String,
    pub user_id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub created_at: NaiveDateTime,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewGeoloc{
    pub id: String,
    pub user_id: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "geolocs"]
pub struct UpdateGeoloc {
    pub id: String,
    pub user_id: String,
    pub latitude: f64,
    pub longitude: f64,
}

pub fn find_today(pool: &PoolType, _user_id: Uuid) -> Result<GeolocsResponse, ApiError> {
    use crate::schema::geolocs::dsl::*;

    let not_found = format!("geoloc {} not found", _user_id);
    let conn = pool.get()?;
    let geoloc = geolocs
        .filter(user_id.eq(_user_id.to_string()))
        //.first::<Geoloc>(&conn)
        .load(&conn)?;

    Ok(geoloc.into())
}

/// Create a new geoloc
pub fn create(pool: &PoolType, new_geoloc: &Geoloc) -> Result<GeolocResponse, ApiError> {
    use crate::schema::geolocs::dsl::*;

    let conn = pool.get()?;
    diesel::insert_into(geolocs).values(new_geoloc).execute(&conn)?;
    Ok(new_geoloc.clone().into())
}


impl From<NewGeoloc> for Geoloc {
    fn from(geoloc: NewGeoloc) -> Self {
        Geoloc {
            id: geoloc.id,
            user_id: geoloc.user_id,
            latitude: geoloc.latitude,
            longitude: geoloc.longitude,
            created_at: Utc::now().naive_utc(),
        }
    }
}