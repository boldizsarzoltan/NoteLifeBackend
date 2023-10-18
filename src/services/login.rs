use bcrypt::BcryptResult;

use crate::applications::general::WEB_APPLICATION;
use crate::auth::hash::verify_password;
use crate::dto::users::{GetTokenRefreshDTO, LoginResponseDTO, TokenRefreshDTO, UserLoginDTO};
use crate::repository::sessions::{create_new_session_by_old, create_session, delete_by_id, delete_session_by_id, find_by_access_token, invalidate_by_user_and_application, Session};
use crate::repository::sessions_refresh::{create_new_refresh_by_old, create_session_refresh, delete_refresh_session_by_id, find_by_refresh_token, invalidate_refresh_by_user_and_application, SessionRefresh};
use crate::repository::types::RepositoryResult;
use crate::repository::user::{get_by_email, get_by_user_name, User};
use crate::services::types::ServiceResult;

pub fn user_login(user_info: UserLoginDTO) -> ServiceResult<LoginResponseDTO, String> {
    let email: Option<String> = user_info.user_name;
    let user_name: Option<String> = user_info.email;
    let password_data: String = user_info.password;
    match user_name {
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
        },
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
        }
    }
}

pub fn refresh_user_token(
    token_refresh_dto: GetTokenRefreshDTO,
    user_id_data: i32,
    header_access_token: String
) -> ServiceResult<TokenRefreshDTO, String> {
    let new_session = find_by_access_token(header_access_token);
    match new_session {
        RepositoryResult::Err(error) => {
            error!("{}", error);
            return ServiceResult::Err("Access or refresh token invalid".to_string());
        },
        RepositoryResult::Ok(ok_session) => {
            if ok_session.user_id != user_id_data {
                return ServiceResult::Err("Access or refresh token invalid".to_string());
            }
            if ok_session.is_active.is_none() {
                delete_session_by_id(ok_session.id);
            }
            if !ok_session.is_active.unwrap() {
                invalidate_by_user_and_application(user_id_data, WEB_APPLICATION.to_string());
                return ServiceResult::Err("Access or refresh token invalid".to_string());
            }
            let new_refresh_token = find_by_refresh_token(token_refresh_dto.refresh_token);
            match new_refresh_token {
                RepositoryResult::Err(error) => {
                    error!("{}", error);
                    return ServiceResult::Err("Access or refresh token invalid".to_string());
                },
                RepositoryResult::Ok(ok_refresh_session) => {
                    if ok_refresh_session.user_id != user_id_data {
                        return ServiceResult::Err("Access or refresh token invalid".to_string());
                    }
                    if ok_refresh_session.is_active.is_none() {
                        delete_session_by_id(ok_session.id);
                        delete_refresh_session_by_id(ok_refresh_session.id);
                    }
                    if !ok_session.is_active.unwrap() {
                        invalidate_by_user_and_application(user_id_data, WEB_APPLICATION.to_string());
                        invalidate_refresh_by_user_and_application(user_id_data, WEB_APPLICATION.to_string());
                        return ServiceResult::Err("Access or refresh token invalid".to_string());
                    }
                    return generate_new_tokens(ok_session, ok_refresh_session);
                }
            }
        }
    }
}
fn generate_new_tokens(old_session: Session, old_session_refresh: SessionRefresh) -> ServiceResult<TokenRefreshDTO, String> {
    let new_session = create_new_session_by_old(old_session);
    match new_session {
        RepositoryResult::Err(error) => {
            error!("{}", error);
            return ServiceResult::Err("Access or refresh token generation failed".to_string());
        },
        RepositoryResult::Ok(ok_session) => {
            let new_refresh_session = create_new_refresh_by_old(old_session_refresh);
            match new_refresh_session {
                RepositoryResult::Err(error) => {
                    error!("{}", error);
                    return ServiceResult::Err("Access or refresh token generation failed".to_string());
                },
                RepositoryResult::Ok(ok_refresh_session) => {
                    return ServiceResult::Ok(TokenRefreshDTO {
                        refresh_token: ok_refresh_session.refresh_token,
                        access_token: ok_session.access_token,
                    });
                }
            }
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