use crate::database::PoolType;
use crate::errors::ApiError;
use crate::handlers::event::{EventResponse, EventsResponse};
use crate::schema::events;
use chrono::{NaiveDateTime, Utc, NaiveDate};
use diesel::prelude::*;
use uuid::Uuid;
use crate::models::subscription::Subscription;
use diesel::dsl::sql;


#[derive(Clone, Debug, Serialize, Associations, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
#[belongs_to(Subscription)]
#[table_name = "events"]
pub struct Event {
    pub id: String,
    pub family_id: String,
    pub subscription_id: String,
    pub place_id: String,
    pub user_id: String,
    pub message: String,
    pub day: String,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_by: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewEvent {
    pub id: String,
    pub family_id: String,
    pub subscription_id: String,
    pub place_id: String,
    pub user_id: String,
    pub day: String,
    pub message: String,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "events"]
pub struct UpdateEvent {
    pub id: String,
    pub family_id: String,
    pub subscription_id: String,
    pub place_id: String,
    pub user_id: String,
    pub day: String,
    pub message: String,
    pub updated_by: String,
}

pub fn get_all(pool: &PoolType) -> Result<EventsResponse, ApiError> {
    use crate::schema::events::dsl::*;

    let conn = pool.get()?;
    let all = events.load(&conn)?;

    Ok(all.into())
}

pub fn get_all_by_family_id(pool: &PoolType, _family_id: Uuid, _day: &String) -> Result<EventsResponse, ApiError> {
    use crate::schema::events::dsl::*;

    println!("passage get_all_by_family_id");
    println!("{:?}", _family_id);

    let conn = pool.get()?;
    let all = events
        .filter(family_id.eq(_family_id.to_string()))
        .filter(day.eq(_day.to_string()))
        .load(&conn)?;
    
    println!("passage get_all_by_family_id");
    println!("{:?}", _family_id);

    Ok(all.into())
}

pub fn get_all_by_family_user_place_sub(
    pool: &PoolType, 
    _family_id: Uuid, 
    _place_id: Uuid, 
    _user_id: Uuid, 
    _subscription_id: Uuid, 
) -> Result<EventsResponse, ApiError> {
    use crate::schema::events::dsl::*;

    let dt = Utc::now().naive_utc();
    println!("dt {:?}", dt);

    let conn = pool.get()?;
    let all: Vec<Event> = events
        .filter(family_id.eq(_family_id.to_string()))
        .filter(place_id.eq(_place_id.to_string()))
        .filter(user_id.eq(_user_id.to_string()))
        .filter(subscription_id.eq(_subscription_id.to_string()))
        //.filter(created_at.gt(NaiveDate::from_ymd(dt.year(), dt.month(), dt.day()).and_hms(0, 0, 0)))
        .filter(sql(r#""events"."created_at" > CURRENT_DATE + interval '1 hour'"#))
        .load(&conn)?;

        println!("**********************************");
        println!("get_all_by_family_user_place_sub");
        println!("all{:?}", all);
        println!("**********************************");

    Ok(all.into())
}



pub fn find(pool: &PoolType, event_id: Uuid) -> Result<EventResponse, ApiError> {
    use crate::schema::events::dsl::*;

    let not_found = format!("event {} not found", event_id);
    let conn = pool.get()?;
    let event = events
        .filter(id.eq(event_id.to_string()))
        .first::<Event>(&conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(event.into())
}

pub fn create(pool: &PoolType, new_event: &Event) -> Result<EventResponse, ApiError> {
    use crate::schema::events::dsl::*;
    let conn = pool.get()?;

    diesel::insert_into(events).values(new_event).execute(&conn)?;
    Ok(new_event.clone().into())
}

pub fn update(pool: &PoolType, update_event: &UpdateEvent) -> Result<EventResponse, ApiError> {
    use crate::schema::events::dsl::*;

    let conn = pool.get()?;
    diesel::update(events)
        .filter(id.eq(update_event.id.clone()))
        .set(update_event)
        .execute(&conn)?;
    find(&pool, Uuid::parse_str(&update_event.id)?)
}

pub fn delete(pool: &PoolType, event_id: Uuid) -> Result<(), ApiError> {
    use crate::schema::events::dsl::*;

    let conn = pool.get()?;
    diesel::delete(events)
        .filter(id.eq(event_id.to_string()))
        .execute(&conn)?;
    Ok(())
}

impl From<NewEvent> for Event {
    fn from(event: NewEvent) -> Self {
        Event {
            id: event.id,
            family_id: event.family_id,
            place_id: event.place_id,
            user_id: event.user_id,
            subscription_id: event.subscription_id,
            day: event.day,
            message: event.message,
            created_by: event.created_by,
            created_at: Utc::now().naive_utc(),
            updated_by: event.updated_by,
            updated_at: Utc::now().naive_utc(),
        }
    }
}