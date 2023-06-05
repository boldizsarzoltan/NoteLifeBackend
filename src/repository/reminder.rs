use crate::repository::connection::get_connection;
use crate::schema::reminders as reminder_table;
use crate::schema::reminders::dsl::*;
use diesel::prelude::{QueryResult, Selectable, Insertable};
use crate::repository::types::{RepositoryResult};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::dto::reminder::{NewReminderDTO, ReminderDTO};
use diesel::RunQueryDsl;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = reminder_table)]
pub struct Reminder {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}


#[derive(Insertable, Debug)]
#[diesel(table_name = reminder_table)]
pub struct NewReminder {
    pub title: String,
    pub description: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}


pub fn get_all_reminders() -> RepositoryResult<Vec<Reminder>, String> {
    let connection = &mut get_connection();
    let results = reminders.load::<Reminder>(connection);
    match results {
        QueryResult::Err(error) => {
            error!("{}", error);
            RepositoryResult::Err(String::from("Database fetch failed"))
        }
        QueryResult::Ok(query_result) => RepositoryResult::Ok(query_result),
    }
}

pub fn insert_reminder(new_reminder_dto: NewReminderDTO) -> RepositoryResult<Reminder, String> {
    let new_reminder: NewReminder = new_reminder_dto.into();
    let new_reminders = vec![new_reminder];
    let connection = &mut get_connection();
    let results: Result<Vec<Reminder>, diesel::result::Error> = diesel::insert_into(reminders)
        .values(&new_reminders)
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

pub fn update_reminder(update_reminder: ReminderDTO) -> RepositoryResult<Reminder, String> {
    let connection = &mut get_connection();

    let updated_row = diesel::update(reminders.filter(id.eq(update_reminder.id)))
        .set((
            title.eq(update_reminder.title.clone()),
            description.eq(update_reminder.description.clone()),
            start_time.eq(update_reminder.start_time.clone()),
            end_time.eq(update_reminder.end_time.clone())
        ))
        .get_result(connection)
        .expect("Cannot update reminder");
    RepositoryResult::Ok(updated_row)
}

pub fn delete_reminder(reminder_id: i32) -> RepositoryResult<String, String>  {
    let connection = &mut get_connection();
    let num_deleted = diesel::delete(reminders.filter(id.eq(reminder_id)))
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