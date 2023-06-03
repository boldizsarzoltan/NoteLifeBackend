use crate::repository::connection::get_connection;
use crate::schema::reminders as reminder_table;
use crate::schema::reminders::dsl::*;
use diesel::prelude::{QueryResult, Selectable, Insertable};
use crate::repository::types::{RepositoryResult};
use chrono::NaiveDateTime;
use diesel::Queryable;
use crate::dto::reminder::NewReminderDTO;
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

pub fn get_all() -> RepositoryResult<Vec<Reminder>, String> {
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

pub fn insert(new_reminder_dto: NewReminderDTO) -> RepositoryResult<Reminder, String> {
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

// pub fn update(update_todo: Todo) -> Todo {
//     let connection = &mut get_connection();

//     let todo = diesel::update(todos.filter(id.eq(update_todo.id)))
//         .set((
//             title.eq(update_todo.title.clone()),
//             visibility.eq(update_todo.visibility),
//             description.eq(update_todo.description.clone()),
//         ))
//         .get_result(connection)
//         .expect("Cannot update todo");

//     return todo;
// }

// pub fn delete(todo_id: i32) -> bool {
//     let connection = &mut get_connection();
//     let num_deleted = diesel::delete(todos.filter(id.eq(todo_id)))
//         .execute(connection)
//         .expect("Error deleting todo");

//     num_deleted != 0
// }