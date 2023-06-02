// @generated automatically by Diesel CLI.

diesel::table! {
    reminders (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        start_time -> Timestamp,
        end_time -> Timestamp,
    }
}
