use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::{respond_json, respond_ok};
use crate::models::subscription::{create, delete, find, get_all_by_family_id, get_all, update, NewSubscription, UpdateSubscription, Subscription};
use crate::validate::validate;
use actix_web::web::{block, Data, HttpResponse, Json, Path};
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;
use rayon::prelude::*;


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

    #[validate(length(
        min = 7,
        message = "days is required and must be at least 3 characters"
    ))]
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

    #[validate(length(
        min = 7,
        message = "days is required and must be at least 3 characters"
    ))]
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

pub async fn create_subscription(
    pool: Data<PoolType>,
    params: Json<CreateSubscriptionRequest>,
) -> Result<Json<SubscriptionResponse>, ApiError> {
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
    place_id: Path<Uuid>,
    pool: Data<PoolType>,
    params: Json<UpdateSubscriptionRequest>,
) -> Result<Json<SubscriptionResponse>, ApiError> {
    validate(&params)?;

    let update_subscription= UpdateSubscription{
        id: place_id.to_string(),
        family_id: params.family_id.to_string(),
        user_id: params.user_id.to_string(),
        place_id: params.place_id.to_string(),
        days: params.days.to_string(),
        updated_by: place_id.to_string(),
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
