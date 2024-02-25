use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use crate::repository::reminder::{get_all_reminders_by_user_id, insert_reminder, update_reminder, delete_reminder};
use crate::dto::reminder::{ReminderDTO, NewReminderDTO};
use crate::repository::types::RepositoryResult;
use crate::auth::auth_guard::AuthenticatedUser;

#[get("/")]
pub async fn get_test() -> Json<String> {
    Json(String::from("OK"))
}

#[options("/all")]
pub async fn get_all_reminders_options_endpoint() -> String {
    String::from("Cso tesooo")
}

#[get("/all")]
pub async fn get_all_reminders_endpoint(user: AuthenticatedUser) -> Result<Json<Vec<ReminderDTO>>, BadRequest<String>> {
    let reminders = get_all_reminders_by_user_id(user.user_id);
    println!("{:?}", user);
    match reminders {
        RepositoryResult::Ok(reminders_from_database) => {
            let reminder_dtos: Vec<ReminderDTO> = reminders_from_database.into_iter().map(|reminder|reminder.into()).collect();
            Ok(Json(reminder_dtos))
        },
        RepositoryResult::Err(message) => Err(BadRequest(Some(message)))
    }
}


#[post("/", format = "json", data = "<new_reminder>")]
pub async fn add_reminder_endpoint(new_reminder: Json<NewReminderDTO>, user: AuthenticatedUser) -> Result<Json<ReminderDTO>, BadRequest<String>> {
    let new_agenda_struct = new_reminder.into_inner();
    let reminder = insert_reminder(new_agenda_struct, user.user_id);
    match reminder {
        RepositoryResult::Ok(reminders_from_database) => {
            Ok(Json(reminders_from_database.into()))
        },
        RepositoryResult::Err(message) => Err(BadRequest(Some(message)))
    }
}

#[options("/")]
pub async fn add_reminder_options_endpoint() -> String {
    String::from("Ok")
}


#[put("/", data = "<update_reminder_variable>")]
pub async fn update_reminder_endpoint(update_reminder_variable: Json<ReminderDTO>, user: AuthenticatedUser) -> Result<Json<ReminderDTO>, BadRequest<String>> {

    let reminder = update_reminder(update_reminder_variable.into_inner(), user.user_id);
    match reminder {
        RepositoryResult::Ok(reminders_from_database) => {
            Ok(Json(reminders_from_database.into()))
        },
        RepositoryResult::Err(message) => Err(BadRequest(Some(message)))
    }
}

#[options("/<reminder_id>")]
pub async fn delete_reminder_options_endpoint(reminder_id: i32) -> String {
    String::from("Ok")
}

#[delete("/<reminder_id>")]
pub async fn delete_reminder_endpoint(reminder_id: i32, user: AuthenticatedUser) -> Result<Json<String>, BadRequest<String>> {
    let reminder: RepositoryResult<String, String> = delete_reminder(reminder_id, user.user_id);
    match reminder {
        RepositoryResult::Ok(response) => Ok(Json(response)),
        RepositoryResult::Err(message) => Err(BadRequest(Some(message)))
    }
}
