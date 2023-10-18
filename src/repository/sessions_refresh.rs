use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, QueryResult, Selectable};
use diesel::prelude::*;
use diesel::RunQueryDsl;
use crate::auth::hash::generate_random_string;

use crate::repository::connection::get_connection;
use crate::repository::types::RepositoryResult;
use crate::schema::app_user_refresh as session_refresh_table;
use crate::schema::app_user_refresh::dsl::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = session_refresh_table)]
pub struct SessionRefresh {
    pub id: i32,
    pub user_id: i32,
    pub refresh_token: String,
    pub application_identifier: String,
    pub is_active: Option<bool>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}


#[derive(Insertable, Debug)]
#[diesel(table_name = session_refresh_table)]
pub struct NewSessionRefresh {
    pub user_id: i32,
    pub refresh_token: String,
    pub application_identifier: String,
    pub is_active: Option<bool>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}


pub fn find_by_refresh_token(session_token: String) -> RepositoryResult<SessionRefresh, String> {
    let connection = &mut get_connection();
    let result = app_user_refresh
        .filter(refresh_token.eq(session_token))
        .first::<SessionRefresh>(connection);
    match result {
        QueryResult::Err(error) => {
            error!("{}", error);
            RepositoryResult::Err(String::from("Database fetch failed"))
        }
        QueryResult::Ok(query_result) => RepositoryResult::Ok(query_result),
    }
}

pub fn refresh_by_refresh_token(refresh_token_data: String) -> RepositoryResult<SessionRefresh, String> {
    let connection = &mut get_connection();
    let result = app_user_refresh
        .filter(refresh_token.eq(refresh_token_data))
        .first::<SessionRefresh>(connection);
    match result {
        QueryResult::Err(error) => {
            error!("{}", error);
            RepositoryResult::Err(String::from("Database fetch failed"))
        }
        QueryResult::Ok(query_result) => RepositoryResult::Ok(query_result),
    }
}
pub fn create_new_refresh_by_old(old_refresh_token: SessionRefresh )
        -> RepositoryResult<SessionRefresh, String> {
    let now = chrono::Utc::now();
    let session_refresh_end = now.checked_add_signed(get_session_refresh_duration());
    if session_refresh_end.is_none() {
        return RepositoryResult::Err(String::from("New session time calculation failed"));
    }
    let session_to_be_inserted =  NewSessionRefresh {
        user_id: old_refresh_token.user_id,
        refresh_token: String::from(""),
        application_identifier: old_refresh_token.application_identifier,
        is_active: Some(true),
        start_time: now.naive_utc(),
        end_time: session_refresh_end.unwrap().naive_utc(),
    };
    let sessions_to_be_inserted = vec![session_to_be_inserted];
    let connection = &mut get_connection();
    let results: Result<Vec<SessionRefresh>, diesel::result::Error> =
        diesel::insert_into(app_user_refresh)
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

fn get_session_refresh_duration() -> chrono::Duration {
    return chrono::Duration::minutes(60);
}

pub fn create_session_refresh(user_id_data: i32, application_identifier_data: String)
    -> RepositoryResult<SessionRefresh, String> {
    let now = chrono::Utc::now();
    let session_refresh_end = now.checked_add_signed(get_session_refresh_duration());
    let new_session_refresh = NewSessionRefresh {
        user_id: user_id_data,
        refresh_token: generate_random_string(64),
        application_identifier: application_identifier_data,
        is_active: Some(true),
        start_time: now.naive_utc(),
        end_time: session_refresh_end.unwrap().naive_utc(),
    };
    let new_session_refreshes = vec![new_session_refresh];
    let connection = &mut get_connection();
    let results: Result<Vec<SessionRefresh>, diesel::result::Error> =
        diesel::insert_into(app_user_refresh)
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
pub fn delete_refresh_session_by_id(refresh_session_id: i32) -> RepositoryResult<String, String> {
    let connection = &mut get_connection();

    let num_deleted =
        diesel::delete(app_user_refresh.filter(id.eq(refresh_session_id)))
            .execute(connection)
            .expect("Cannot delete session");
    match num_deleted {
        1 => {
            RepositoryResult::Ok(String::from("Ok"))
        }
        num => {
            error!("Number of deleted rows is {}", num);
            RepositoryResult::Err(String::from("Database delete failed"))
        },
    }
}

pub fn invalidate_by_user(user_id_data: i32) -> RepositoryResult<SessionRefresh, String> {
    let connection = &mut get_connection();

    let updated_row =
        diesel::update(app_user_refresh.filter(user_id.eq(user_id_data)))
        .set(is_active.eq(false))
        .get_result(connection)
        .expect("Cannot update session refresh");
    RepositoryResult::Ok(updated_row)
}

pub fn invalidate_refresh_by_user_and_application(user_id_data: i32, application_identifier_data: String)
    -> RepositoryResult<SessionRefresh, String> {
    let connection = &mut get_connection();

    let updated_row =
        diesel::update(app_user_refresh
                .filter(user_id.eq(user_id_data))
                .filter(application_identifier.eq(application_identifier_data))
            )
            .set(is_active.eq(false))
            .get_result(connection)
            .expect("Cannot update session refresh");
    RepositoryResult::Ok(updated_row)
}