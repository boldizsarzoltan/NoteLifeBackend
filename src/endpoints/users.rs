use rocket::serde::json::Json;
use crate::repository::user::{get_all_users, insert_user, login_by_email, login_by_user_name, User};
use crate::dto::users::{UserDTO, NewUserDTO, UserLoginDTO, LoginResponseDTO};
use crate::repository::types::RepositoryResult;
use chrono::Utc;

#[get("/all")]
pub async fn get_all_users_endpoint() -> Result<Json<Vec<UserDTO>>, String> {
    let reminders = get_all_users();
    println!("{}", Utc::now().naive_utc());
    match reminders {
        RepositoryResult::Ok(reminders_from_database) => {
            let user_dtos: Vec<UserDTO> = reminders_from_database.into_iter().map(|reminder|reminder.into()).collect();
            Ok(Json(user_dtos))
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
    let email: Option<String> = user_info.user_name;
    let user_name: Option<String> = user_info.email;
    let password_data: String = user_info.password;
    match user_name {
        None => {
            match email {
                None => {
                    return Err(String::from("No username or email given"));

                },
                Some(email_with_value) => {
                    let result = login_by_email(email_with_value, password_data);
                    match result {
                        RepositoryResult::Ok(ok_result) => {
                            return Ok(Json(ok_result.into()));
                        },
                        RepositoryResult::Err(err_message) => {
                            return Err(err_message);
                        },
                    }
                }
            }
        },
        Some(user_name_with_value) => {
            let result =login_by_user_name(user_name_with_value, password_data);
            match result {
                RepositoryResult::Ok(ok_result) => {
                    return Ok(Json(ok_result.into()));
                },
                RepositoryResult::Err(err_message) => {
                    return Err(err_message);
                },
            }
        },
    }
}