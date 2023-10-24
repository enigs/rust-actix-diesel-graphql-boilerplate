// @generated automatically by Diesel CLI.

diesel::table! {
    settings (id) {
        #[max_length = 32]
        id -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        #[max_length = 100]
        module -> Nullable<Varchar>,
        content -> Jsonb,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    settings,
);
