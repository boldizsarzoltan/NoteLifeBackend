#[macro_use] extern crate rocket;

mod cors;
mod repository;
mod endpoints;
mod dto;
mod schema;
mod auth;
mod applications;
mod services;

use endpoints::reminders::{get_all_reminders_endpoint, add_reminder_endpoint, update_reminder_endpoint, delete_reminder_endpoint, get_test};
use endpoints::users::{get_all_users_endpoint, add_user_endpoint, login_user_endpoint};
use cors::Cors;
use crate::endpoints::users::refresh_token;


#[launch]
async fn rocket() -> _ {
    repository::migrations::run_migrations();
    rocket::build().attach(Cors)
    .mount("/", routes![get_test])
    .mount("/reminder", routes![get_all_reminders_endpoint, add_reminder_endpoint, update_reminder_endpoint, delete_reminder_endpoint])
    .mount("/user", routes![get_all_users_endpoint, add_user_endpoint, login_user_endpoint, refresh_token])
}