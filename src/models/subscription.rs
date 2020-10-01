use crate::database::PoolType;
use crate::errors::ApiError;
use crate::handlers::subscription::{SubscriptionResponse, SubscriptionsResponse, SubscriptionFilledResponse, SubscriptionsFilledResponse};
use crate::schema::subscriptions;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct Subscription {
    pub id: String,
    pub family_id: String,
    pub place_id: String,
    pub user_id: String,
    pub days: String,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_by: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewSubscription {
    pub id: String,
    pub family_id: String,
    pub place_id: String,
    pub user_id: String,
    pub days: String,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "subscriptions"]
pub struct UpdateSubscription {
    pub id: String,
    pub family_id: String,
    pub place_id: String,
    pub user_id: String,
    pub days: String,
    pub updated_by: String,
}

pub fn get_all(pool: &PoolType) -> Result<SubscriptionsResponse, ApiError> {
    use crate::schema::subscriptions::dsl::subscriptions;

    let conn = pool.get()?;
    let all = subscriptions.load(&conn)?;

    Ok(all.into())
}

pub fn get_all_by_family_id(pool: &PoolType, _family_id: Uuid) -> Result<SubscriptionsResponse, ApiError> {
    use crate::schema::subscriptions::dsl::*;

    let conn = pool.get()?;
    let all = subscriptions
        .filter(family_id.eq(_family_id.to_string()))
        .load(&conn)?;

    Ok(all.into())
}

pub fn get_all_by_family_id_and_place_id(pool: &PoolType, _family_id: Uuid, _place_id: Uuid) -> Result<SubscriptionsFilledResponse, ApiError> {
    use crate::schema::subscriptions::dsl::*;

    let conn = pool.get()?;
    let all = subscriptions
        .filter(family_id.eq(_family_id.to_string()))
        .filter(place_id.eq(_place_id.to_string()))
        .load(&conn)?;

    Ok(all.into())
}

pub fn find(pool: &PoolType, subscription_id: Uuid) -> Result<SubscriptionResponse, ApiError> {
    use crate::schema::subscriptions::dsl::{id, subscriptions};

    let not_found = format!("subscription {} not found", subscription_id);
    let conn = pool.get()?;
    let subscription = subscriptions
        .filter(id.eq(subscription_id.to_string()))
        .first::<Subscription>(&conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(subscription.into())
}

pub fn create(pool: &PoolType, new_subscription: &Subscription) -> Result<SubscriptionResponse, ApiError> {
    use crate::schema::subscriptions::dsl::subscriptions;
    let conn = pool.get()?;

    diesel::insert_into(subscriptions).values(new_subscription).execute(&conn)?;
    Ok(new_subscription.clone().into())
}

pub fn update(pool: &PoolType, update_subscription: &UpdateSubscription) -> Result<SubscriptionResponse, ApiError> {
    use crate::schema::subscriptions::dsl::{id, subscriptions};

    let conn = pool.get()?;
    diesel::update(subscriptions)
        .filter(id.eq(update_subscription.id.clone()))
        .set(update_subscription)
        .execute(&conn)?;
    find(&pool, Uuid::parse_str(&update_subscription.id)?)
}

pub fn delete(pool: &PoolType, subscription_id: Uuid) -> Result<(), ApiError> {
    use crate::schema::subscriptions::dsl::{id, subscriptions};

    let conn = pool.get()?;
    diesel::delete(subscriptions)
        .filter(id.eq(subscription_id.to_string()))
        .execute(&conn)?;
    Ok(())
}

impl From<NewSubscription> for Subscription {
    fn from(subscription: NewSubscription) -> Self {
        Subscription {
            id: subscription.id,
            family_id: subscription.family_id,
            place_id: subscription.place_id,
            user_id: subscription.user_id,
            days: subscription.days,
            created_by: subscription.created_by,
            created_at: Utc::now().naive_utc(),
            updated_by: subscription.updated_by,
            updated_at: Utc::now().naive_utc(),
        }
    }
}