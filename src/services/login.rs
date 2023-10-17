use bcrypt::BcryptResult;
use crate::applications::general::WEB_APPLICATION;
use crate::auth::hash::verify_password;
use crate::dto::users::{LoginResponseDTO, UserLoginDTO};
use crate::repository::sessions::{create_session, delete_by_id};
use crate::repository::sessions_refresh::create_session_refresh;
use crate::repository::types::RepositoryResult;
use crate::repository::user::{get_by_email, get_by_user_name, User};
use crate::services::types::ServiceResult;

pub fn user_login(user_info: UserLoginDTO) -> ServiceResult<LoginResponseDTO, String> {
    let email: Option<String> = user_info.user_name;
    let user_name: Option<String> = user_info.email;
    let password_data: String = user_info.password;
    match user_name {
        None => {
            match email {
                None => {
                    return ServiceResult::Err(String::from("No username or email given"));
                }
                Some(email_with_value) => {
                    let result = get_by_email(email_with_value);
                    match result {
                        RepositoryResult::Ok(ok_result) => {
                            return verify_password_local(ok_result, password_data.clone());
                        }
                        RepositoryResult::Err(err_message) => {
                            return ServiceResult::Err(err_message);
                        }
                    }
                }
            }
        },
        Some(user_name_with_value) => {
            let result = get_by_user_name(user_name_with_value);
            match result {
                RepositoryResult::Ok(ok_result) => {
                    return verify_password_local(ok_result, password_data.clone());
                }
                RepositoryResult::Err(err_message) => {
                    return ServiceResult::Err(err_message);
                }
            }
        }
    }
}

pub fn verify_password_local(user:User, password_data:String) -> ServiceResult<LoginResponseDTO, String> {
    let result = verify_password(password_data, user.password.as_str());
    match result {
        BcryptResult::Err(error) => {
            error!("{}", error);
            return ServiceResult::Err(String::from("Login failed"));
        }
        BcryptResult::Ok(user_response) => {
            if user_response {
                return generate_login_dto(user);
            }
            return ServiceResult::Err(String::from("Login failed"));
        }
    }
}

fn generate_login_dto(user: User) -> ServiceResult<LoginResponseDTO, String> {
    let session = create_session(
        user.id,
        WEB_APPLICATION.to_string()
    );
    match session {
        RepositoryResult::Err(error) => {
            return ServiceResult::Err(error);
        },
        RepositoryResult::Ok(ok_session) => {
            let refresh_session = create_session_refresh(
                user.id,
                WEB_APPLICATION.to_string()
            );
            match refresh_session {
                RepositoryResult::Err(error) => {
                    delete_by_id(ok_session.id);
                    return ServiceResult::Err(error);
                },
                RepositoryResult::Ok(ok_refresh_session) => {
                    let login_response = LoginResponseDTO {
                        role: user.role,
                        access_token: ok_session.access_token,
                        refresh_token: ok_refresh_session.refresh_token,
                    };
                    return ServiceResult::Ok(login_response);
                }
            }
        }
    }
}