// @generated automatically by Diesel CLI.

diesel::table! {
    app_users (id) {
        id -> Int4,
        user_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::table! {
    reminders (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        start_time -> Timestamp,
        end_time -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    app_users,
    reminders,
);
