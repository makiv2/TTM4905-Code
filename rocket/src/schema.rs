// @generated automatically by Diesel CLI.

diesel::table! {
    proofs (id) {
        id -> Int4,
        proof -> Text,
        company -> Varchar,
        message -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        message -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    proofs,
    users,
);
