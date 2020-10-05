use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::{respond_json, respond_ok};
use crate::models::event::{ get_all_by_family_user_place_sub, create, delete, find, get_all_by_family_id, get_all, update, NewEvent, UpdateEvent, Event};
use crate::validate::validate;
use actix_web::web::{block, Data, HttpResponse, Json, Path};
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;
use rayon::prelude::*;


#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct EventResponse {
    pub id: Uuid,
    pub family_id: Uuid,
    pub subscription_id: Uuid,
    pub place_id: Uuid,
    pub user_id: Uuid,
    pub day: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct EventsResponse(pub Vec<EventResponse>);

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateEventRequest {
    #[validate(length(
        min = 3,
        message = "family_id is required and must be at least 3 characters"
    ))]
    pub family_id: String,

    #[validate(length(
        min = 3,
        message = "subscription_id is required and must be at least 3 characters"
    ))]
    pub subscription_id: String,

    #[validate(length(
        min = 3,
        message = "user_id is required and must be at least 3 characters"
    ))]
    pub user_id: String,

    #[validate(length(
        min = 3,
        message = "place_id is required and must be at least 3 characters"
    ))]
    pub place_id: String,

    #[validate(length(
        min = 1,
        message = "message is required and must be at least 3 characters"
    ))]
    pub message: String,

    #[validate(length(
        min = 1,
        message = "day is required and must be at least 3 characters"
    ))]
    pub day: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateEventRequest {
    #[validate(length(
        min = 3,
        message = "family_id is required and must be at least 3 characters"
    ))]
    pub family_id: String,

    #[validate(length(
        min = 3,
        message = "user_id is required and must be at least 3 characters"
    ))]
    pub user_id: String,

    #[validate(length(
        min = 3,
        message = "place_id is required and must be at least 3 characters"
    ))]
    pub place_id: String,

    #[validate(length(
        min = 3,
        message = "subscription_id is required and must be at least 3 characters"
    ))]
    pub subscription_id: String,

    #[validate(length(
        min = 1,
        message = "message is required and must be at least 3 characters"
    ))]
    pub message: String,

    #[validate(length(
        min = 1,
        message = "day is required and must be at least 3 characters"
    ))]
    pub day: String,
}

pub async fn get_event(
    event_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<Json<EventResponse>, ApiError> {
    let event = block(move || find(&pool, *event_id)).await?;
    respond_json(event)
}

pub async fn get_events(pool: Data<PoolType>) -> Result<Json<EventsResponse>, ApiError> {
    let events = block(move || get_all(&pool)).await?;
    respond_json(events)
}

#[derive(Deserialize)]
pub struct PathByFamilyID {
    family_id: Uuid,
    day: String,
}

pub async fn get_events_by_family_id(path: Path<PathByFamilyID>, pool: Data<PoolType>) -> Result<Json<EventsResponse>, ApiError> {
    let events = block(move || get_all_by_family_id(&pool, path.family_id, &path.day)).await?;
    respond_json(events)
}

#[derive(Deserialize)]
pub struct PathByFamilyPlaceUserSub {
    family_id: Uuid,
    subscription_id: Uuid,
    place_id: Uuid,
    user_id: Uuid,
}

pub async fn get_events_by_family_place_user_user(path: Path<PathByFamilyPlaceUserSub>, pool: Data<PoolType>) -> Result<Json<EventsResponse>, ApiError> {
    let events = block(move || get_all_by_family_user_place_sub(&pool, path.family_id, path.place_id, path.user_id, path.subscription_id)).await?;
    println!("events {:?}", events);
    respond_json(events)
}

pub async fn create_event(
    pool: Data<PoolType>,
    params: Json<CreateEventRequest>,
) -> Result<Json<EventResponse>, ApiError> {
    //println!("create_subscription");
    //println!("{:?}", params);

    validate(&params)?;

    let event_id = Uuid::new_v4();
    let new_event: Event = NewEvent {
        id: event_id.to_string(),
        subscription_id: params.subscription_id.to_string(),
        family_id: params.family_id.to_string(),
        user_id: params.user_id.to_string(),
        place_id: params.place_id.to_string(),
        day: params.day.to_string(),
        message: params.message.to_string(),
        created_by: event_id.to_string(),
        updated_by: event_id.to_string(),
    }
    .into();
    let event = block(move || create(&pool, &new_event)).await?;
    respond_json(event.into())
}

pub async fn update_event(
    event_id: Path<Uuid>,
    pool: Data<PoolType>,
    params: Json<UpdateEventRequest>,
) -> Result<Json<EventResponse>, ApiError> {
    validate(&params)?;

    let update_event= UpdateEvent{
        id: event_id.to_string(),
        subscription_id: params.subscription_id.to_string(),
        family_id: params.family_id.to_string(),
        user_id: params.user_id.to_string(),
        place_id: params.place_id.to_string(),
        day: params.day.to_string(),
        message: params.message.to_string(),
        updated_by: event_id.to_string(),
    };
    let event = block(move || update(&pool, &update_event)).await?;
    respond_json(event.into())
}

/// Delete a user
pub async fn delete_event(
    event_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<HttpResponse, ApiError> {
    block(move || delete(&pool, *event_id)).await?;
    respond_ok()
}

impl From<Event> for EventResponse {
    fn from(event: Event) -> Self {
        println!("//////////////////////////////");
        println!("EventResponse");
        println!("event.day {:?}", event.day);
        println!("event.message {:?}", event.message);
        println!("//////////////////////////////");
        EventResponse {
            id: Uuid::parse_str(&event.id).unwrap(),
            subscription_id: Uuid::parse_str(&event.subscription_id).unwrap(),
            family_id: Uuid::parse_str(&event.family_id).unwrap(),
            user_id: Uuid::parse_str(&event.user_id).unwrap(),
            place_id: Uuid::parse_str(&event.place_id).unwrap(),
            day: event.day.to_string(),
            message: event.message.to_string(),
        }
    }
}

impl From<Vec<Event>> for EventsResponse {
    fn from(events: Vec<Event>) -> Self {
        EventsResponse(events.into_par_iter().map(|event| event.into()).collect())
    }
}
