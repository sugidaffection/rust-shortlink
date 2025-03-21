// @generated automatically by Diesel CLI.

diesel::table! {
    link_clicks (id) {
        id -> Uuid,
        link_id -> Uuid,
        clicked_at -> Timestamptz,
        referrer -> Nullable<Text>,
        user_agent -> Nullable<Text>,
        ip_address -> Nullable<Varchar>,
        country_code -> Nullable<Varchar>,
    }
}

diesel::table! {
    short_links (id) {
        id -> Uuid,
        owner_id -> Nullable<Uuid>,
        anonymous_owner_id -> Nullable<Uuid>,
        serial_id -> Int8,
        hash -> Nullable<Varchar>,
        long_url -> Text,
        is_private -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        expires_at -> Nullable<Timestamptz>,
        click_count -> Int8,
        title -> Nullable<Varchar>,
        description -> Nullable<Text>,
        status -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(link_clicks -> short_links (link_id));
diesel::joinable!(short_links -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(
    link_clicks,
    short_links,
    users,
);
