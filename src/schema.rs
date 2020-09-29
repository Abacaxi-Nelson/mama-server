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
    }
}

joinable!(places -> families (family_id));
joinable!(subscriptions -> families (family_id));
joinable!(subscriptions -> places (place_id));
joinable!(subscriptions -> users (user_id));
joinable!(users -> families (family_id));

allow_tables_to_appear_in_same_query!(
    families,
    places,
    subscriptions,
    users,
);
