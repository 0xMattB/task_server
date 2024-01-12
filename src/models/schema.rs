// @generated automatically by Diesel CLI.

diesel::table! {
    entries (id) {
        #[max_length = 255]
        id -> Varchar,
        username -> Text,
        year -> Text,
        month -> Text,
        day -> Text,
        task -> Text,
        reminder -> Nullable<Text>,
        expired -> Text,
        created -> Timestamp,
        updated -> Timestamp,
        user_id -> Text,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 255]
        id -> Varchar,
        username -> Text,
        password -> Text,
        email -> Text,
        utc_offset -> Nullable<Text>,
    }
}

diesel::joinable!(entries -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    entries,
    users,
);
