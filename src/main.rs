#[macro_use] extern crate rocket;

mod cors;
mod repository;
mod endpoints;
mod dto;
mod schema;

use endpoints::endpoints::{get_all_reminders_endpoint, add_reminder_endpoint, update_reminder_endpoint, delete_reminder_endpoint, get_test};
use cors::Cors;


#[launch]
async fn rocket() -> _ {
    rocket::build().attach(Cors)
    .mount("/", routes![get_test])
    .mount("/reminder", routes![get_all_reminders_endpoint, add_reminder_endpoint, update_reminder_endpoint, delete_reminder_endpoint])
}