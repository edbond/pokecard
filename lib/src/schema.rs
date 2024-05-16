// @generated automatically by Diesel CLI.

diesel::table! {
    cards (id) {
        id -> Integer,
        title -> Text,
        image -> Nullable<Binary>,
        price -> Nullable<Double>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        url -> Nullable<Text>,
        image_url -> Nullable<Text>,
    }
}

diesel::table! {
    sqlean_define (name) {
        name -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        body -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    cards,
    sqlean_define,
);
