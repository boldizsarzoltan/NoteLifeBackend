use rocket::serde::json::Json;
use crate::repository::reminder::{get_all, insert};
use crate::dto::reminder::{ReminderDTO, NewReminderDTO};
use crate::endpoints::types::EndpointResult;
use crate::repository::types::RepositoryResult;

#[get("/all")]
pub async fn get_all_reminders() -> Result<Json<Vec<ReminderDTO>>, String> {
    let reminders = get_all();
    match reminders {
        RepositoryResult::Ok(reminders_from_database) => {
            let reminder_dtos: Vec<ReminderDTO> = reminders_from_database.into_iter().map(|reminder|reminder.into()).collect();
            Ok(rocket::serde::json::Json(reminder_dtos))
        },
        RepositoryResult::Err(message) => Err(message)
    }
}


#[post("/", format = "json", data = "<new_agenda>")]
pub async fn add_reminder(new_agenda: Json<NewReminderDTO>) -> Result<Json<ReminderDTO>, String> {
    let new_agenda_struct = new_agenda.into_inner();
    let reminder = insert(new_agenda_struct);
    match reminder {
        RepositoryResult::Ok(reminders_from_database) => {
            Ok(rocket::serde::json::Json(reminders_from_database.into()))
        },
        RepositoryResult::Err(message) => Err(message)
    }
}