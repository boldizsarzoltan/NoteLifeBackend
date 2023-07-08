use rocket::serde::json::Json;
use crate::repository::reminder::{get_all_reminders, insert_reminder, update_reminder, delete_reminder};
use crate::dto::reminder::{ReminderDTO, NewReminderDTO};
use crate::repository::types::RepositoryResult;
use chrono::Utc;

#[get("/")]
pub async fn get_test() -> Json<String> {
    rocket::serde::json::Json(String::from("OK"))
}

#[get("/all")]
pub async fn get_all_reminders_endpoint() -> Result<Json<Vec<ReminderDTO>>, String> {
    let reminders = get_all_reminders();
    println!("{}", Utc::now().naive_utc());
    match reminders {
        RepositoryResult::Ok(reminders_from_database) => {
            let reminder_dtos: Vec<ReminderDTO> = reminders_from_database.into_iter().map(|reminder|reminder.into()).collect();
            Ok(rocket::serde::json::Json(reminder_dtos))
        },
        RepositoryResult::Err(message) => Err(message)
    }
}


#[post("/", format = "json", data = "<new_reminder>")]
pub async fn add_reminder_endpoint(new_reminder: Json<NewReminderDTO>) -> Result<Json<ReminderDTO>, String> {
    let new_agenda_struct = new_reminder.into_inner();
    let reminder = insert_reminder(new_agenda_struct);
    match reminder {
        RepositoryResult::Ok(reminders_from_database) => {
            Ok(rocket::serde::json::Json(reminders_from_database.into()))
        },
        RepositoryResult::Err(message) => Err(message)
    }
}


#[put("/", data = "<update_reminder_variable>")]
pub async fn update_reminder_endpoint(update_reminder_variable: Json<ReminderDTO>) -> Result<Json<ReminderDTO>, String> {
    let reminder = update_reminder(update_reminder_variable.into_inner());
    match reminder {
        RepositoryResult::Ok(reminders_from_database) => {
            Ok(rocket::serde::json::Json(reminders_from_database.into()))
        },
        RepositoryResult::Err(message) => Err(message)
    }
}


#[delete("/<reminder_id>")]
pub async fn delete_reminder_endpoint(reminder_id: i32) -> Result<String, String> {
    let reminder: RepositoryResult<String, String> = delete_reminder(reminder_id);
    match reminder {
        RepositoryResult::Ok(response) => Ok(response),
        RepositoryResult::Err(message) => Err(message)
    }
}
