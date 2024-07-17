// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "action"))]
    pub struct Action;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "clean_state"))]
    pub struct CleanState;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "point", schema = "pg_catalog"))]
    pub struct Point;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role"))]
    pub struct Role;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "state"))]
    pub struct State;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "topic"))]
    pub struct Topic;
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
    use super::sql_types::Action;

    histories (id) {
        id -> Int4,
        water_closet_id -> Int4,
        datetime -> Timestamp,
        action -> Action,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Point;

    places (id) {
        id -> Int4,
        group_id -> Int4,
        coordonates -> Point,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::State;
    use super::sql_types::Topic;

    reports (id) {
        id -> Int4,
        user_id -> Int4,
        water_closet_id -> Int4,
        datetime -> Timestamp,
        state -> State,
        topic -> Topic,
        comment -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Role;

    users (id) {
        id -> Int4,
        mail -> Varchar,
        phone -> Varchar,
        role -> Role,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CleanState;

    water_closets (id) {
        id -> Int4,
        group_id -> Int4,
        is_disabled -> Bool,
        is_door_opened -> Bool,
        is_door_locked -> Bool,
        clean_state -> CleanState,
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
