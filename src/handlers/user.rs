use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::{respond_json, respond_ok};
use crate::models::user::{create, delete, find, get_all_by_family_id, get_all, update, NewUser, UpdateUser, User};
use crate::validate::validate;
use actix_web::web::{block, Data, HttpResponse, Json, Path};
use rayon::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UserResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub family_id: Option<String>,
    pub role: Option<String>,
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UsersResponse(pub Vec<UserResponse>);

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(
        min = 3,
        message = "first_name is required and must be at least 3 characters"
    ))]
    pub first_name: String,

    #[validate(length(
        min = 3,
        message = "last_name is required and must be at least 3 characters"
    ))]
    pub last_name: String,

    #[validate(email(message = "email must be a valid email"))]
    pub email: String,

    #[validate(length(
        min = 6,
        message = "password is required and must be at least 6 characters"
    ))]
    pub password: String,

    #[validate(length(
        min = 6,
        message = "token is required and must be at least 6 characters"
    ))]
    pub token: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(
        min = 3,
        message = "first_name is required and must be at least 3 characters"
    ))]
    pub first_name: String,

    #[validate(length(
        min = 3,
        message = "last_name is required and must be at least 3 characters"
    ))]
    pub last_name: String,

    #[validate(email(message = "email must be a valid email"))]
    pub email: String,

    pub role: Option<String>,
    pub family_id: Option<String>,

    #[validate(length(
        min = 6,
        message = "token is required and must be at least 6 characters"
    ))]
    pub token: String,
}

/// Get a user
pub async fn get_user(
    user_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = block(move || find(&pool, *user_id)).await?;
    respond_json(user)
}

/// Get all users
pub async fn get_users(pool: Data<PoolType>) -> Result<Json<UsersResponse>, ApiError> {
    let users = block(move || get_all(&pool)).await?;
    respond_json(users)
}

#[derive(Deserialize)]
pub struct PathByFamilyID {
    family_id: Uuid
}

/// Get all users by family_id
pub async fn get_users_by_family_id(path: Path<PathByFamilyID>, pool: Data<PoolType>) -> Result<Json<UsersResponse>, ApiError> {
    let users = block(move || get_all_by_family_id(&pool, path.family_id)).await?;
    respond_json(users)
}

/// Create a user
pub async fn create_user(
    pool: Data<PoolType>,
    params: Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    println!("create_user");
    validate(&params)?;
    println!("create_user2");

    // temporarily use the new user's id for created_at/updated_at
    // update when auth is added
    let user_id = Uuid::new_v4();
    let new_user: User = NewUser {
        id: user_id.to_string(),
        first_name: params.first_name.to_string(),
        last_name: params.last_name.to_string(),
        email: params.email.to_string(),
        password: params.password.to_string(),
        created_by: user_id.to_string(),
        updated_by: user_id.to_string(),
        token: params.token.to_string(),
    }
    .into();
    let user = block(move || create(&pool, &new_user)).await?;
    respond_json(user.into())
}

/// Update a user
pub async fn update_user(
    user_id: Path<Uuid>,
    pool: Data<PoolType>,
    params: Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    validate(&params)?;

    // temporarily use the user's id for updated_at
    // update when auth is added
    let update_user = UpdateUser {
        id: user_id.to_string(),
        first_name: params.first_name.to_string(),
        last_name: params.last_name.to_string(),
        email: params.email.to_string(),
        updated_by: user_id.to_string(),
        family_id: params.family_id.clone(),
        role: params.role.clone(),
        token: params.token.clone(),
    };
    let user = block(move || update(&pool, &update_user)).await?;
    respond_json(user.into())
}

/// Delete a user
pub async fn delete_user(
    user_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<HttpResponse, ApiError> {
    block(move || delete(&pool, *user_id)).await?;
    respond_ok()
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: Uuid::parse_str(&user.id).unwrap(),
            first_name: user.first_name.to_string(),
            last_name: user.last_name.to_string(),
            email: user.email.to_string(),
            role: user.role,
            family_id: user.family_id,
            token: user.token,
        }
    }
}

impl From<Vec<User>> for UsersResponse {
    fn from(users: Vec<User>) -> Self {
        UsersResponse(users.into_par_iter().map(|user| user.into()).collect())
    }
}
