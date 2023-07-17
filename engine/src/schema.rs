// @generated automatically by Diesel CLI.

diesel::table! {
    fingerprints (id) {
        id -> Integer,
        hash -> BigInt,
        loc -> Integer,
        lang -> Text,
        path -> Text,
        fs_id -> Text,
    }
}
