// @generated automatically by Diesel CLI.

diesel::table! {
    stories (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        rating -> Nullable<Int4>,
        comment -> Nullable<Varchar>,
        progress -> Nullable<Int4>,
        length -> Nullable<Int4>,
        link -> Nullable<Varchar>,
    }
}
