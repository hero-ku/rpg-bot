// @generated automatically by Diesel CLI.

diesel::table! {
    characters (name, owner) {
        name -> Text,
        owner -> Int8,
    }
}

diesel::table! {
    stats (char_name, char_owner, name) {
        char_name -> Text,
        char_owner -> Int8,
        name -> Text,
        value -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        active_char -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    characters,
    stats,
    users,
);
