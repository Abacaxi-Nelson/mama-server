table! {
    events (id) {
        id -> Varchar,
        family_id -> Varchar,
        subscription_id -> Varchar,
        place_id -> Varchar,
        user_id -> Varchar,
        message -> Varchar,
        day -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

table! {
    families (id) {
        id -> Varchar,
        nom -> Varchar,
        code -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

table! {
    geolocs (id) {
        id -> Varchar,
        user_id -> Varchar,
        latitude -> Float8,
        longitude -> Float8,
        created_at -> Timestamp,
    }
}

table! {
    places (id) {
        id -> Varchar,
        name -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
        family_id -> Varchar,
    }
}

table! {
    subscriptions (id) {
        id -> Varchar,
        family_id -> Varchar,
        place_id -> Varchar,
        user_id -> Varchar,
        days -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
        family_id -> Nullable<Varchar>,
        role -> Nullable<Varchar>,
        token -> Varchar,
    }
}

joinable!(events -> families (family_id));
joinable!(events -> places (place_id));
joinable!(events -> subscriptions (subscription_id));
joinable!(events -> users (user_id));
joinable!(geolocs -> users (user_id));
joinable!(places -> families (family_id));
joinable!(subscriptions -> families (family_id));
joinable!(subscriptions -> places (place_id));
joinable!(subscriptions -> users (user_id));
joinable!(users -> families (family_id));

allow_tables_to_appear_in_same_query!(
    events,
    families,
    geolocs,
    places,
    subscriptions,
    users,
);
