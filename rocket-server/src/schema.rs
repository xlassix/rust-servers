// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        #[max_length = 36]
        id -> Bpchar,
        title -> Nullable<Varchar>,
        body -> Varchar,
        #[max_length = 36]
        user_id -> Bpchar,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 36]
        id -> Bpchar,
        name -> Nullable<Varchar>,
        username -> Varchar,
        password -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
