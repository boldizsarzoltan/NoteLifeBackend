use chrono::Utc;
use rocket::serde::json::Json;

use crate::dto::users::{LoginResponseDTO, NewUserDTO, UserDTO, UserLoginDTO};
use crate::repository::types::RepositoryResult;
use crate::repository::user::{get_all_users, insert_user, User};
use crate::services::login::user_login;
use crate::services::types::ServiceResult;

#[get("/all")]
pub async fn get_all_users_endpoint() -> Result<Json<Vec<UserDTO>>, String> {
    let reminders = get_all_users();
    println!("{}", Utc::now().naive_utc());
    match reminders {
        RepositoryResult::Ok(reminders_from_database) => {
            let user_dto: Vec<UserDTO> = reminders_from_database.into_iter().map(|reminder|reminder.into()).collect();
            Ok(Json(user_dto))
        },
        RepositoryResult::Err(message) => Err(message)
    }
}


#[post("/", format = "json", data = "<new_user>")]
pub async fn add_user_endpoint(new_user: Json<NewUserDTO>) -> Result<Json<UserDTO>, String> {
    let new_user_struct: NewUserDTO = new_user.into_inner();
    let reminder:RepositoryResult<User, String> = insert_user(new_user_struct);
    match reminder {
        RepositoryResult::Ok(reminders_from_database) => {
            Ok(Json(reminders_from_database.into()))
        },
        RepositoryResult::Err(message) => Err(message)
    }
}

#[post("/login", format = "json", data = "<user>")]
pub async fn login_user_endpoint(user: Json<UserLoginDTO>) -> Result<Json<LoginResponseDTO>, String> {
    let user_info: UserLoginDTO = user.into_inner();
    let user_login_data = user_login(user_info);
    match user_login_data {
        ServiceResult::Ok(ok_user_login_data) => {
            Ok(Json(ok_user_login_data))
        },
        ServiceResult::Err(message) => Err(message)
    }
}