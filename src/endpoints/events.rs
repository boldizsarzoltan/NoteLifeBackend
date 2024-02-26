use chrono::Utc;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;

use crate::auth::auth_guard::AuthenticatedUser;
use crate::dto::event::{EventDTO, NewEventDTO};
use crate::repository::events::{delete_event, Event, get_events_by_user_id, insert_event, update_event};
use crate::repository::types::RepositoryResult;
use crate::services::types::ServiceResult;

#[get("/all")]
pub async fn get_all_user_events_endpoint(user: AuthenticatedUser) -> Result<Json<Vec<EventDTO>>, BadRequest<Option<String>>> {
    let events = get_events_by_user_id(user.user_id);
    println!("{}", Utc::now().naive_utc());
    match events {
        RepositoryResult::Ok(events_from_database) => {
            let event_dto: Vec<EventDTO> = events_from_database.into_iter().map(|event|event.into()).collect();
            Ok(Json(event_dto))
        }
        RepositoryResult::Err(message) => Err(BadRequest(Some(message)))
    }
}

#[post("/", format = "json", data = "<new_event>")]
pub async fn add_user_event_endpoint(new_event: Json<NewEventDTO>, user: AuthenticatedUser) -> Result<Json<EventDTO>, BadRequest<Option<String>>> {
    let new_event_struct: NewEventDTO = new_event.into_inner();
    let reminder:RepositoryResult<Event, String> = insert_event(new_event_struct, user.user_id);
    match reminder {
        RepositoryResult::Ok(reminders_from_database) => {
            Ok(Json(reminders_from_database.into()))
        },
        RepositoryResult::Err(message) => Err(BadRequest(Some(message)))
    }
}

#[put("/", format = "json", data = "<event>")]
pub async fn update_user_event_endpoint(event: Json<EventDTO>, user: AuthenticatedUser) -> Result<Json<EventDTO>, BadRequest<Option<String>>> {
    let user_event: EventDTO = event.into_inner();
    let user_event_data = update_event(user_event, user.user_id);
    match user_event_data {
        RepositoryResult::Ok(ok_event_data) => {
            Ok(Json(ok_event_data.into()))
        }
        RepositoryResult::Err(message) => Err(BadRequest(Some(message)))
    }
}
#[options("/all")]
pub async fn get_all_event_options_endpoint() -> String {
    String::from("Ok")
}

#[options("/")]
pub async fn create_event_options_endpoint() -> String {
    String::from("Ok")
}

#[options("/<event_id>")]
pub async fn delete_event_options_endpoint(event_id: i32) -> String {
    String::from("Ok")
}

#[delete("/<event_id>")]
pub(crate) async fn delete_event_endpoint(event_id: i32, user: AuthenticatedUser) -> Result<Json<String>, BadRequest<Option<String>>> {
    let reminder: RepositoryResult<String, String> = delete_event(event_id, user.user_id);
    match reminder {
        RepositoryResult::Ok(response) => Ok(Json(response)),
        RepositoryResult::Err(message) => Err(BadRequest(Some(message)))
    }
}