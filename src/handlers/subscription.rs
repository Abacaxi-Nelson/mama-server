use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::{respond_json, respond_ok};
use crate::models::subscription::{ get_all_by_family_id_and_user_id_and_days_events, get_all_by_family_id_and_user_id_and_days, get_all_by_family_id_and_place_id, create, delete, find, get_all_by_family_id, get_all, update, NewSubscription, UpdateSubscription, Subscription};
use crate::validate::validate;
use actix_web::web::{block, Data, HttpResponse, Json, Path};
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;
use rayon::prelude::*;
use super::event::{EventResponse, EventsResponse};
use crate::models::event::Event;


#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SubscriptionResponse {
    pub id: Uuid,
    pub family_id: Uuid,
    pub user_id: Uuid,
    pub place_id: Uuid,
    pub days: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SubscriptionsResponse(pub Vec<SubscriptionResponse>);

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateSubscriptionRequest {
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

    pub days: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateSubscriptionRequest {
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

    pub days: String,
}

pub async fn get_subscription(
    subscription_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<Json<SubscriptionResponse>, ApiError> {
    let subscription = block(move || find(&pool, *subscription_id)).await?;
    respond_json(subscription)
}

pub async fn get_subscriptions(pool: Data<PoolType>) -> Result<Json<SubscriptionsResponse>, ApiError> {
    let subscriptions = block(move || get_all(&pool)).await?;
    respond_json(subscriptions)
}

#[derive(Deserialize)]
pub struct PathByFamilyID {
    family_id: Uuid
}

pub async fn get_subscriptions_by_family_id(path: Path<PathByFamilyID>, pool: Data<PoolType>) -> Result<Json<SubscriptionsResponse>, ApiError> {
    let subscriptions = block(move || get_all_by_family_id(&pool, path.family_id)).await?;
    respond_json(subscriptions)
}

#[derive(Deserialize)]
pub struct PathByFamilyIDPlaceID {
    family_id: Uuid,
    place_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SubscriptionFilledResponse {
    pub id: Uuid,
    pub family_id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub place_id: Uuid,
    pub place_name: String,
    pub days: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SubscriptionsFilledResponse(pub Vec<SubscriptionFilledResponse>);

pub async fn get_subscriptions_by_family_id_and_place_id(path: Path<PathByFamilyIDPlaceID>, pool: Data<PoolType>) -> Result<Json<SubscriptionsResponse>, ApiError> {
    println!("get_subscriptions_by_family_id_and_place_id");
    let subscriptions = block(move || get_all_by_family_id_and_place_id(&pool, path.family_id, path.place_id)).await?;
    println!("get_subscriptions_by_family_id_and_place_id 2");
    respond_json(subscriptions)
}

#[derive(Deserialize)]
pub struct PathByFamilyIDUserIDDays {
    family_id: Uuid,
    user_id: Uuid,
    days: String,
}

pub async fn search_by_family_user_days(path: Path<PathByFamilyIDUserIDDays>, pool: Data<PoolType>) -> Result<Json<SubscriptionsResponse>, ApiError> {
    println!("get_subscriptions_by_family_id_and_user_id_and_days");
    let subscriptions = block(move || get_all_by_family_id_and_user_id_and_days(&pool, path.family_id, path.user_id, &path.days)).await?;
    println!("subscriptions {:?} ", subscriptions);
    println!("get_subscriptions_by_family_id_and_user_id_and_days 2");
    respond_json(subscriptions)
}

pub async fn search_by_family_user_days_events(path: Path<PathByFamilyIDUserIDDays>, pool: Data<PoolType>) -> Result<Json<SubscriptionsEventResponse>, ApiError> {
    println!("get_subscriptions_by_family_id_and_user_id_and_days");
    let subscriptions = block(move || get_all_by_family_id_and_user_id_and_days_events(&pool, path.family_id, path.user_id, &path.days)).await?;
    println!("subscriptions {:?} ", subscriptions);
    println!("get_subscriptions_by_family_id_and_user_id_and_days 2");
    respond_json(subscriptions)
}


pub async fn create_subscription(
    pool: Data<PoolType>,
    params: Json<CreateSubscriptionRequest>,
) -> Result<Json<SubscriptionResponse>, ApiError> {
    //println!("create_subscription");
    //println!("{:?}", params);

    validate(&params)?;

    let subscription_id = Uuid::new_v4();
    let new_subscription: Subscription = NewSubscription {
        id: subscription_id.to_string(),
        family_id: params.family_id.to_string(),
        user_id: params.user_id.to_string(),
        place_id: params.place_id.to_string(),
        days: params.days.to_string(),
        created_by: subscription_id.to_string(),
        updated_by: subscription_id.to_string(),
    }
    .into();
    let subscription = block(move || create(&pool, &new_subscription)).await?;
    respond_json(subscription.into())
}

pub async fn update_subscription(
    sub_id: Path<Uuid>,
    pool: Data<PoolType>,
    params: Json<UpdateSubscriptionRequest>,
) -> Result<Json<SubscriptionResponse>, ApiError> {
    validate(&params)?;

    let update_subscription= UpdateSubscription{
        id: sub_id.to_string(),
        family_id: params.family_id.to_string(),
        user_id: params.user_id.to_string(),
        place_id: params.place_id.to_string(),
        days: params.days.to_string(),
        updated_by: sub_id.to_string(),
    };
    let subscription = block(move || update(&pool, &update_subscription)).await?;
    respond_json(subscription.into())
}

/// Delete a user
pub async fn delete_subscription(
    subscription_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<HttpResponse, ApiError> {
    block(move || delete(&pool, *subscription_id)).await?;
    respond_ok()
}

impl From<Subscription> for SubscriptionResponse {
    fn from(subscription: Subscription) -> Self {
        SubscriptionResponse {
            id: Uuid::parse_str(&subscription.id).unwrap(),
            family_id: Uuid::parse_str(&subscription.family_id).unwrap(),
            user_id: Uuid::parse_str(&subscription.user_id).unwrap(),
            place_id: Uuid::parse_str(&subscription.place_id).unwrap(),
            days: subscription.days.to_string(),
        }
    }
}

impl From<Vec<Subscription>> for SubscriptionsResponse {
    fn from(subscriptions: Vec<Subscription>) -> Self {
        SubscriptionsResponse(subscriptions.into_par_iter().map(|subscription| subscription.into()).collect())
    }
}


#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SubscriptionEventResponse {
    pub s: SubscriptionResponse ,
    pub e: EventResponse 
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SubscriptionsEventResponse(pub Vec<SubscriptionEventResponse>);

impl From<(Subscription, Event)> for SubscriptionEventResponse {
    fn from((subscription, event): (Subscription, Event)) -> Self {
        SubscriptionEventResponse{
            s: SubscriptionResponse {
                id: Uuid::parse_str(&subscription.id).unwrap(),
                family_id: Uuid::parse_str(&subscription.family_id).unwrap(),
                user_id: Uuid::parse_str(&subscription.user_id).unwrap(),
                place_id: Uuid::parse_str(&subscription.place_id).unwrap(),
                days: subscription.days.to_string(),
            },
            e: EventResponse {
                id: Uuid::parse_str(&event.id).unwrap(),
                family_id: Uuid::parse_str(&event.family_id).unwrap(),
                subscription_id: Uuid::parse_str(&event.subscription_id).unwrap(),
                place_id: Uuid::parse_str(&event.place_id).unwrap(),
                user_id: Uuid::parse_str(&event.user_id).unwrap(),
                day: event.day.to_string(),
                message: event.message.to_string()
            }
        }
    }
}

impl From<Vec<(Subscription, Event)>> for SubscriptionsEventResponse {
    fn from(all: Vec<(Subscription, Event)>) -> Self {
        /*SubscriptionsEventResponse(
            all.into_par_iter()
            .map(|subscription, event| 
                (subscription.into(),event.into())
            ).collect()
        )*/
        SubscriptionsEventResponse(
            all
            .iter()
            .map(
                |t| 
                //(t.0.into(),t.1.into())
                SubscriptionEventResponse{
                    //s: t.0.into(), e: t.1.into()
                    s: t.0.clone().into(), e: t.1.clone().into()
                }
            )
            .collect()
        )
    }
}