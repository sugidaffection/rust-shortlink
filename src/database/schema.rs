table! {
    short_link (id) {
        id -> Uuid,
        owner_id -> Uuid,
        hash -> Varchar,
        long_url -> Varchar,
        is_private -> Nullable<Bool>,
        created_at -> Timestamp,
        uid -> Int4,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(short_link -> users (owner_id));

allow_tables_to_appear_in_same_query!(
    short_link,
    users,
);
