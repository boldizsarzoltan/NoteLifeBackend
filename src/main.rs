#[macro_use] extern crate rocket;

mod cors;
mod repository;
mod endpoints;
mod dto;
mod schema;

use endpoints::endpoints::{get_all_reminders, add_reminder};
use cors::Cors;


#[launch]
async fn rocket() -> _ {
    rocket::build().attach(Cors)
    .mount("/reminder", routes![get_all_reminders, add_reminder])
}