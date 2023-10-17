use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, QueryResult, Selectable};
use diesel::prelude::*;
use diesel::RunQueryDsl;
use crate::auth::hash::generate_random_string;

use crate::repository::connection::get_connection;
use crate::repository::types::RepositoryResult;
use crate::schema::app_user_sessions as session_tables;
use crate::schema::app_user_sessions::dsl::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = session_tables)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub access_token: String,
    pub application_identifier: String,
    pub is_active: Option<bool>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}


#[derive(Insertable, Debug)]
#[diesel(table_name = session_tables)]
pub struct NewSession {
    pub user_id: i32,
    pub access_token: String,
    pub application_identifier: String,
    pub is_active: Option<bool>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}


pub fn find_by_access_token(session_token: String) -> RepositoryResult<Session, String> {
    let connection = &mut get_connection();
    let result = app_user_sessions
        .filter(access_token.eq(session_token))
        .first::<Session>(connection);
    match result {
        QueryResult::Err(error) => {
            error!("{}", error);
            RepositoryResult::Err(String::from("Database fetch failed"))
        }
        QueryResult::Ok(query_result) => RepositoryResult::Ok(query_result),
    }
}

fn create_new_session_by_old(old_session: Session ) -> RepositoryResult<Session, String> {
    let now = chrono::Utc::now();
    let session_end = now.checked_add_signed(get_session_duration());
    if session_end.is_none() {
        return RepositoryResult::Err(String::from("New session time calculation failed"));
    }
    let session_to_be_inserted =  NewSession {
        user_id: old_session.user_id,
        access_token: String::from(""),
        application_identifier: old_session.application_identifier,
        is_active: Some(true),
        start_time: now.naive_utc(),
        end_time: session_end.unwrap().naive_utc(),
    };
    let sessions_to_be_inserted = vec![session_to_be_inserted];
    let connection = &mut get_connection();
    let results: Result<Vec<Session>, diesel::result::Error> = diesel::insert_into(app_user_sessions)
        .values(&sessions_to_be_inserted)
        .get_results(connection);
    match results {
        QueryResult::Err(error) => {
            error!("{}", error);
            RepositoryResult::Err(String::from("Database insert failed"))
        }
        QueryResult::Ok(query_result) => {
            RepositoryResult::Ok(query_result.into_iter().nth(0).unwrap())
        }
    }
}

fn get_session_duration() -> chrono::Duration {
    return chrono::Duration::minutes(30);
}

pub fn create_session(user_id_data: i32, application_identifier_data: String)
                              -> RepositoryResult<Session, String> {
    let now = chrono::Utc::now();
    let session_refresh_end = now.checked_add_signed(get_session_duration());
    let new_session = NewSession {
        user_id: user_id_data,
        access_token: generate_random_string(64),
        application_identifier: application_identifier_data,
        is_active: Some(true),
        start_time: now.naive_utc(),
        end_time: session_refresh_end.unwrap().naive_utc(),
    };
    let new_session_refreshes = vec![new_session];
    let connection = &mut get_connection();
    let results: Result<Vec<Session>, diesel::result::Error> =
        diesel::insert_into(app_user_sessions)
            .values(&new_session_refreshes)
            .get_results(connection);
    match results {
        QueryResult::Err(error) => {
            error!("{}", error);
            RepositoryResult::Err(String::from("Database insert failed"))
        }
        QueryResult::Ok(query_result) => {
            RepositoryResult::Ok(query_result.into_iter().nth(0).unwrap())
        }
    }
}

pub fn invalidate_by_user(user_id_data: i32) -> RepositoryResult<Session, String> {
    let connection = &mut get_connection();

    let updated_row =
        diesel::update(app_user_sessions.filter(user_id.eq(user_id_data)))
            .set(is_active.eq(None::<bool>))
            .get_result(connection)
            .expect("Cannot update reminder");
    RepositoryResult::Ok(updated_row)
}

pub fn delete_by_id(session_id: i32) -> RepositoryResult<String, String> {
    let connection = &mut get_connection();

    let num_deleted =
        diesel::delete(app_user_sessions.filter(id.eq(session_id)))
            .execute(connection)
            .expect("Cannot delete reminder");
    match num_deleted {
        1 => {
            RepositoryResult::Ok(String::from("Ok"))
        }
        num => {
            error!("Numebr of delted rows is {}", num);
            RepositoryResult::Err(String::from("Database delete failed"))
        },
    }
}

pub fn invalidate_by_user_and_application(user_id_data: i32, application_identifier_data: String)
                                          -> RepositoryResult<Session, String> {
    let connection = &mut get_connection();

    let updated_row =
        diesel::update(app_user_sessions
            .filter(user_id.eq(user_id_data))
            .filter(application_identifier.eq(application_identifier_data))
        )
            .set(is_active.eq(None::<bool>))
            .get_result(connection)
            .expect("Cannot update reminder");
    RepositoryResult::Ok(updated_row)
}

pub fn set_inactive_by_user_and_application(user_id_data: i32, application_identifier_data: String)
                                          -> RepositoryResult<Session, String> {
    let connection = &mut get_connection();

    let updated_row =
        diesel::update(app_user_sessions
                .filter(user_id.eq(user_id_data))
                .filter(application_identifier.eq(application_identifier_data))
            )
            .set(is_active.eq(false))
            .get_result(connection)
            .expect("Cannot update reminder");
    RepositoryResult::Ok(updated_row)
}