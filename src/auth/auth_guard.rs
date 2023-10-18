use rocket::http::Status;
use rocket::Request;
use rocket::request::FromRequest;
use rocket::request::Outcome;

use crate::applications::general::{ADMIN, APPLICATIONS, USER};
use crate::auth::types::SessionResult;
use crate::repository::sessions::{find_by_access_token, invalidate_by_user_and_application, Session};
use crate::repository::types::RepositoryResult;
use crate::repository::user::get_user_by_id;

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub user_hash: String,
    pub user_id: i32
}

#[derive(Debug)]
pub struct AuthenticatedAdmin {
    user_hash: String,
    user_id: i32
}

#[derive(Debug)]
pub enum AuthError {
    InvalidRequest,
    InvalidSession,
    InactiveSession,
    ExpiredSession,
    UserDoesNotHaveAccess
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for AuthenticatedUser {
    type Error = AuthError;
    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        if !request.headers().contains("Authorization")  {
            return Outcome::Failure((Status::Unauthorized, AuthError::InvalidRequest));
        }
        let session_token = request.headers().get_one("Authorization");
        if session_token.is_none() {
            return Outcome::Failure((Status::BadRequest, AuthError::InvalidRequest));
        }
        let correct_session_token = session_token.unwrap().to_string();
        let session = find_by_access_token(correct_session_token.clone());
            match session {
            RepositoryResult::Err(error) => {
                error!("{}", error);
                return Outcome::Failure((Status::BadRequest, AuthError::InvalidRequest));
            },
            RepositoryResult::Ok(ok_session) => {
                let auth_user = AuthenticatedUser {
                    user_hash: correct_session_token,
                    user_id: ok_session.user_id
                };
                return Outcome::Success(auth_user);
            }
        }
    }
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for AuthenticatedAdmin {
    type Error = AuthError;
    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        if !request.headers().contains("Authorization") {
            return Outcome::Failure((Status::Unauthorized, AuthError::InvalidRequest));
        }
        let session_token = request.headers().get_one("Authorization");
        if session_token.is_none() {
            return Outcome::Failure((Status::BadRequest, AuthError::InvalidRequest));
        }
        let session = find_by_access_token(session_token.unwrap().to_string());
        match session {
            RepositoryResult::Err(_error) => {
                return Outcome::Failure((Status::BadRequest, AuthError::InvalidRequest));
            },
            RepositoryResult::Ok(current_session) => {
                let verified_session = verify_session(current_session);
                match verified_session {
                    SessionResult::FatalErr(_fatal_error) => {
                        return Outcome::Failure((Status::BadRequest, AuthError::InvalidRequest));
                    },
                    SessionResult::Err(_expired_session) => {
                        return Outcome::Failure((Status::NotAcceptable, AuthError::ExpiredSession));
                    },
                    SessionResult::Ok(correct_session) => {
                        return verify_admin(correct_session);
                    }
                }
            }
        }
    }
}

fn verify_session(session: Session) -> SessionResult<Session, AuthError, AuthError> {
    if session.start_time.timestamp() > session.end_time.timestamp()
        || session.start_time.timestamp() > chrono::Utc::now().timestamp()
    {
        invalidate_by_user_and_application(session.user_id, session.application_identifier);
        return SessionResult::FatalErr(AuthError::InvalidSession);
    }
    if session.is_active.is_none() {
        return SessionResult::FatalErr(AuthError::InvalidSession);
    }
    if !APPLICATIONS.contains(&session.application_identifier.as_str()) {
        invalidate_by_user_and_application(session.user_id, session.application_identifier);
        return SessionResult::FatalErr(AuthError::InvalidSession);
    }
    if !session.is_active.is_none() && !session.is_active.unwrap() {
        return SessionResult::Err(AuthError::InactiveSession);
    }
    if session.end_time.timestamp()  > chrono::Utc::now().timestamp() {
        return SessionResult::Err(AuthError::ExpiredSession);
    }
    return SessionResult::Ok(session);
}

fn verify_admin(session :Session) -> Outcome<AuthenticatedAdmin, AuthError> {
    let user = get_user_by_id(session.user_id);
    match user {
        RepositoryResult::Err(error) => {
            error!("{}", error);
            return Outcome::Failure((Status::BadRequest, AuthError::InvalidRequest));
        },
        RepositoryResult::Ok(user) => {
            if String::from(ADMIN) != user.role {
                return Outcome::Failure((Status::Forbidden, AuthError::UserDoesNotHaveAccess));
            }
            let auth_admin = AuthenticatedAdmin{
                user_hash: session.access_token,
                user_id: session.user_id
            };
            return Outcome::Success(auth_admin);
        }
    }
}

fn verify_user(session :Session) -> Outcome<AuthenticatedUser, AuthError> {
    let user = get_user_by_id(session.user_id);
    match user {
        RepositoryResult::Err(error) => {
            error!("{}", error);
            return Outcome::Failure((Status::BadRequest, AuthError::InvalidRequest));
        },
        RepositoryResult::Ok(user) => {
            if String::from(USER) != user.role && String::from(ADMIN) != user.role {
                return Outcome::Failure((Status::Forbidden, AuthError::UserDoesNotHaveAccess));
            }
            let auth_admin = AuthenticatedUser{
                user_hash: session.access_token,
                user_id: session.user_id
            };
            return Outcome::Success(auth_admin);
        }
    }
}