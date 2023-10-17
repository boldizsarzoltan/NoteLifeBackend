use rocket::serde::json::Json;
use crate::repository::reminder::{get_all_reminders, insert_reminder, update_reminder, delete_reminder};
use crate::dto::reminder::{ReminderDTO, NewReminderDTO};
use crate::repository::types::RepositoryResult;
use crate::auth::auth_guard::AuthenticatedUser;

#[get("/")]
pub async fn get_test() -> Json<String> {
    Json(String::from("OK"))
}

#[get("/all")]
pub async fn get_all_reminders_endpoint(user: AuthenticatedUser) -> Result<Json<Vec<ReminderDTO>>, String> {
    let reminders = get_all_reminders();
    println!("{:?}", user);
    match reminders {
        RepositoryResult::Ok(reminders_from_database) => {
            let reminder_dtos: Vec<ReminderDTO> = reminders_from_database.into_iter().map(|reminder|reminder.into()).collect();
            Ok(Json(reminder_dtos))
        },
        RepositoryResult::Err(message) => Err(message)
    }
}


#[post("/", format = "json", data = "<new_reminder>")]
pub async fn add_reminder_endpoint(new_reminder: Json<NewReminderDTO>, user: AuthenticatedUser) -> Result<Json<ReminderDTO>, String> {
    let new_agenda_struct = new_reminder.into_inner();
    let reminder = insert_reminder(new_agenda_struct);
    match reminder {
        RepositoryResult::Ok(reminders_from_database) => {
            Ok(Json(reminders_from_database.into()))
        },
        RepositoryResult::Err(message) => Err(message)
    }
}


#[put("/", data = "<update_reminder_variable>")]
pub async fn update_reminder_endpoint(update_reminder_variable: Json<ReminderDTO>, user: AuthenticatedUser) -> Result<Json<ReminderDTO>, String> {
    let reminder = update_reminder(update_reminder_variable.into_inner());
    match reminder {
        RepositoryResult::Ok(reminders_from_database) => {
            Ok(Json(reminders_from_database.into()))
        },
        RepositoryResult::Err(message) => Err(message)
    }
}


#[delete("/<reminder_id>")]
pub async fn delete_reminder_endpoint(reminder_id: i32, user: AuthenticatedUser) -> Result<String, String> {
    let reminder: RepositoryResult<String, String> = delete_reminder(reminder_id);
    match reminder {
        RepositoryResult::Ok(response) => Ok(response),
        RepositoryResult::Err(message) => Err(message)
    }
}
