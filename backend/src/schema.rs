// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "actions"))]
    pub struct Actions;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "clean_states"))]
    pub struct CleanStates;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "roles"))]
    pub struct Roles;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "states"))]
    pub struct States;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "topics"))]
    pub struct Topics;
}

diesel::table! {
    groups (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Actions;

    histories (id) {
        id -> Int4,
        water_closet_id -> Int4,
        datetime -> Timestamp,
        action -> Actions,
    }
}

diesel::table! {
    places (id) {
        id -> Int4,
        group_id -> Int4,
        coordonates -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::States;
    use super::sql_types::Topics;

    reports (id) {
        id -> Int4,
        user_id -> Int4,
        water_closet_id -> Int4,
        datetime -> Timestamp,
        state -> States,
        topic -> Topics,
        comment -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Roles;

    users (id) {
        id -> Int4,
        mail -> Varchar,
        phone -> Varchar,
        role -> Roles,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CleanStates;

    water_closets (id) {
        id -> Int4,
        group_id -> Int4,
        is_disabled -> Bool,
        is_door_opened -> Bool,
        is_door_locked -> Bool,
        clean_state -> CleanStates,
    }
}

diesel::joinable!(groups -> users (user_id));
diesel::joinable!(histories -> water_closets (water_closet_id));
diesel::joinable!(places -> groups (group_id));
diesel::joinable!(reports -> users (user_id));
diesel::joinable!(reports -> water_closets (water_closet_id));
diesel::joinable!(water_closets -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    histories,
    places,
    reports,
    users,
    water_closets,
);
