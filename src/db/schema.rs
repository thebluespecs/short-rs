// @generated automatically by Diesel CLI.

diesel::table! {
    urls (id) {
        id -> Int8,
        original_url -> Text,
        #[max_length = 200]
        short_code -> Varchar,
        visits -> Int8,
        expires_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
