// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]


use chrono::NaiveDateTime;
#[derive(Queryable, Debug)]
pub struct AppUserRefresh {
    pub id: i32,
    pub user_id: i32,
    pub refresh_token: String,
    pub application_identifier: String,
    pub is_active: Option<bool>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct AppUserSession {
    pub id: i32,
    pub user_id: i32,
    pub access_token: String,
    pub application_identifier: String,
    pub is_active: Option<bool>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct AppUser {
    pub id: i32,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Queryable, Debug)]
pub struct Reminder {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub user_id: i32,
}

#[derive(Queryable, Debug)]
pub struct UserEvent {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub date_time: NaiveDateTime,
    pub event_user_id: i32,
}
