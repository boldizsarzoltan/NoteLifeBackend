#[macro_use] extern crate rocket;

mod cors;
mod repository;
mod endpoints;
mod dto;
mod schema;
mod auth;
mod applications;
mod services;

use endpoints::reminders::{get_all_reminders_endpoint, add_reminder_endpoint, update_reminder_endpoint, delete_reminder_endpoint, get_test, delete_reminder_options_endpoint, add_reminder_options_endpoint, get_all_reminders_options_endpoint};
use endpoints::users::{get_all_users_endpoint, add_user_endpoint, add_user_options_endpoint, login_user_endpoint};
use cors::Cors;
use crate::endpoints::users::{login_user_options_endpoint, refresh_token};
use dotenvy::dotenv;
use crate::endpoints::events::{add_user_event_endpoint, delete_event_endpoint, get_all_user_events_endpoint, update_user_event_endpoint, get_all_event_options_endpoint, delete_event_options_endpoint, create_event_options_endpoint};


#[launch]
async fn rocket() -> _ {
    match std::env::var("ENV") {
        Ok(ref env) if env == "dev" => dotenv::from_filename(".env.dev").ok(),
        Ok(ref env) if env != "dev" => dotenv::from_filename(".env").ok(),
        _ => dotenv().ok(),
    };
    repository::migrations::run_migrations();
    rocket::build().attach(Cors)
    .mount("/", routes![get_test])
    .mount("/reminder", routes![get_all_reminders_endpoint, get_all_reminders_options_endpoint, add_reminder_options_endpoint, add_reminder_endpoint, update_reminder_endpoint, delete_reminder_endpoint, delete_reminder_options_endpoint])
    .mount("/event", routes![get_all_user_events_endpoint, add_user_event_endpoint,update_user_event_endpoint, delete_event_endpoint, get_all_event_options_endpoint, delete_event_options_endpoint, create_event_options_endpoint])
    .mount("/user", routes![get_all_users_endpoint, add_user_endpoint, add_user_options_endpoint, login_user_endpoint, refresh_token, login_user_options_endpoint])
}