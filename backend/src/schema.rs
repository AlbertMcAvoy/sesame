// @generated automatically by Diesel CLI.

diesel::table! {
    categories (category_id) {
        category_id -> Uuid,
        name -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    categories,
);
