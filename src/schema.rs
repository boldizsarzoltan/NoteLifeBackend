// @generated automatically by Diesel CLI.

diesel::table! {
    app_user_refresh (id) {
        id -> Int4,
        user_id -> Int4,
        refresh_token -> Varchar,
        application_identifier -> Varchar,
        is_active -> Nullable<Bool>,
        start_time -> Timestamp,
        end_time -> Timestamp,
    }
}

diesel::table! {
    app_user_sessions (id) {
        id -> Int4,
        user_id -> Int4,
        access_token -> Varchar,
        application_identifier -> Varchar,
        is_active -> Nullable<Bool>,
        start_time -> Timestamp,
        end_time -> Timestamp,
    }
}

diesel::table! {
    app_users (id) {
        id -> Int4,
        user_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        role -> Varchar,
    }
}

diesel::table! {
    reminders (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        start_time -> Timestamp,
        end_time -> Timestamp,
        user_id -> Int4,
    }
}

diesel::table! {
    user_events (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        date_time -> Timestamp,
        event_user_id -> Int4,
    }
}

diesel::joinable!(app_user_refresh -> app_users (user_id));
diesel::joinable!(app_user_sessions -> app_users (user_id));
diesel::joinable!(reminders -> app_users (user_id));
diesel::joinable!(user_events -> app_users (event_user_id));

diesel::allow_tables_to_appear_in_same_query!(
    app_user_refresh,
    app_user_sessions,
    app_users,
    reminders,
    user_events,
);
