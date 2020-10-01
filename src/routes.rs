//! Place all Actix routes here, multiple route configs can be used and
//! combined.

use crate::handlers::{
    auth::{login, logout},
    health::get_health,
    user::{get_users_by_family_id, create_user, delete_user, get_user, get_users, update_user},
    family::{get_family_by_code, create_family, delete_family, get_family, get_families, update_family},
    place::{get_places_by_family_id, create_place, delete_place, get_place, get_places, update_place},
    subscription::{get_subscriptions_by_family_id_and_place_id,get_subscriptions_by_family_id, create_subscription, delete_subscription, get_subscription, get_subscriptions, update_subscription},
};
use crate::middleware::auth::Auth as AuthMiddleware;
use actix_files::Files;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Healthcheck
        .route("/health", web::get().to(get_health))
        // /api/v1 routes
        .service(
            web::scope("/api/v1")
                // Lock down routes with AUTH Middleware
                .wrap(AuthMiddleware)
                // AUTH routes
                .service(
                    web::scope("/auth")
                        .route("/login", web::post().to(login))
                        .route("/logout", web::get().to(logout)),
                )
                // USER routes
                .service(
                    web::scope("/user")
                        .route("/{id}", web::get().to(get_user))
                        .route("/{id}", web::put().to(update_user))
                        .route("/{id}", web::delete().to(delete_user))
                        .route("", web::get().to(get_users))
                        .route("", web::post().to(create_user))
                        .route("/search_by_family/{family_id}", web::get().to(get_users_by_family_id)),
                )
                // FAMILY routes
                .service(
                    web::scope("/family")
                        .route("/{id}", web::get().to(get_family))
                        .route("/{id}", web::put().to(update_family))
                        .route("/{id}", web::delete().to(delete_family))
                        .route("", web::get().to(get_families))
                        .route("", web::post().to(create_family))
                        .route("/search_by_code/{code}", web::get().to(get_family_by_code)),
                )
                // PLACE routes
                .service(
                    web::scope("/place")
                        .route("/{id}", web::get().to(get_place))
                        .route("/{id}", web::put().to(update_place))
                        .route("/{id}", web::delete().to(delete_place))
                        .route("", web::get().to(get_places))
                        .route("", web::post().to(create_place))
                        .route("search_by_family/{family_id}", web::get().to(get_places_by_family_id)),
                )
                // Subscription routes
                .service(
                    web::scope("/subscription")
                        .route("/{id}", web::get().to(get_subscription))
                        .route("/{id}", web::put().to(update_subscription))
                        .route("/{id}", web::delete().to(delete_subscription))
                        .route("", web::get().to(get_subscriptions))
                        .route("", web::post().to(create_subscription))
                        .route("search_by_family/{family_id}", web::get().to(get_subscriptions_by_family_id))
                        .route("search_by_family_place/{family_id}/{place_id}", web::get().to(get_subscriptions_by_family_id_and_place_id)),
                )
        )
        // Serve secure static files from the static-private folder
        .service(
            web::scope("/secure").wrap(AuthMiddleware).service(
                Files::new("", "./static-secure")
                    .index_file("index.html")
                    .use_last_modified(true),
            ),
        )
        // Serve public static files from the static folder
        .service(
            web::scope("").default_service(
                Files::new("", "./static")
                    .index_file("index.html")
                    .use_last_modified(true),
            ),
        );
}
