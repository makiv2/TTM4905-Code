// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        message -> Text,
    }
}

diesel::table! {
    proofs (id) {
        id -> Int4,
        proof -> Text,
        company -> Varchar,
        message -> Text,
    }
}
