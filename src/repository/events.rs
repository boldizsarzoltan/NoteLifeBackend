use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, QueryResult, Selectable};
use diesel::prelude::*;
use diesel::RunQueryDsl;

use crate::dto::event::{EventDTO, NewEventDTO};
use crate::dto::reminder::ReminderDTO;
use crate::repository::connection::get_connection;
use crate::repository::reminder::Reminder;
use crate::repository::types::RepositoryResult;
use crate::schema::reminders::dsl::reminders;
use crate::schema::reminders::{end_time, start_time, user_id};
use crate::schema::user_events as events_table;
use crate::schema::user_events::dsl::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = events_table)]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub date_time: NaiveDateTime,
    pub event_user_id: i32,
}


#[derive(Insertable, Debug)]
#[diesel(table_name = events_table)]
pub struct NewEvent {
    pub title: String,
    pub description: Option<String>,
    pub date_time: NaiveDateTime,
    pub event_user_id: i32,
}

pub fn insert_event(new_event_dto: NewEventDTO, user_id_data: i32) -> RepositoryResult<Event, String> {
    let new_event: NewEvent = NewEvent {
        title: new_event_dto.title,
        description: new_event_dto.description,
        date_time: new_event_dto.date_time,
        event_user_id: user_id_data
    };
    let new_events = vec![new_event];
    let connection = &mut get_connection();
    let results: Result<Vec<Event>, diesel::result::Error> = diesel::insert_into(user_events)
        .values(&new_events)
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

pub fn update_event(event_dto: EventDTO, user_id_data: i32) -> RepositoryResult<Event, String> {
    let connection = &mut get_connection();

    let updated_row = diesel::update(
        user_events
            .filter(id.eq(event_dto.id))
            .filter(event_user_id.eq(user_id_data))
        )
        .set((
            title.eq(event_dto.title.clone()),
            description.eq(event_dto.description.clone()),
            date_time.eq(event_dto.date_time.clone()),
        ))
        .get_result(connection);
    match updated_row {
        QueryResult::Err(error)=> {
            error!("{}", error);
            RepositoryResult::Err(String::from("Database delete failed"))
        }
        QueryResult::Ok(result)=> {
            return RepositoryResult::Ok(result);
        },
    }
}

pub fn delete_event(event_id: i32, user_id_data: i32) -> RepositoryResult<String, String>  {
    let connection = &mut get_connection();
    let num_deleted = diesel::delete(
        user_events
            .filter(id.eq(event_id))
            .filter(event_user_id.eq(user_id_data))
        )
        .execute(connection)
        .expect("Error deleting todo");

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

pub fn get_events_by_user_id(user_id_data:i32) -> RepositoryResult<Vec<Event>, String> {
    let connection = &mut get_connection();
    let result = user_events
        .filter(event_user_id.eq(user_id_data))
        .load::<Event>(connection);
    match result {
        QueryResult::Err(error) => {
            error!("{}", error);
            return RepositoryResult::Err(String::from("User not found"));
        }
        QueryResult::Ok(user) => {
            return RepositoryResult::Ok(user);
        }
    }
}