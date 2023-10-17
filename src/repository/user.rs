use diesel::prelude::{Insertable, QueryResult, Selectable};
use diesel::prelude::*;
use diesel::RunQueryDsl;

use crate::dto::users::NewUserDTO;
use crate::repository::connection::get_connection;
use crate::repository::types::RepositoryResult;
use crate::schema::app_users as users_table;
use crate::schema::app_users::dsl::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = users_table)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}


#[derive(Insertable, Debug)]
#[diesel(table_name = users_table)]
pub struct NewUser {
    pub user_name: String,
    pub email: String,
    pub role: String,
    pub password: String,
}

pub fn get_all_users() -> RepositoryResult<Vec<User>, String> {
    let connection = &mut get_connection();
    let results = app_users.load::<User>(connection);
    match results {
        QueryResult::Err(error) => {
            error!("{}", error);
            RepositoryResult::Err(String::from("Database fetch failed"))
        }
        QueryResult::Ok(query_result) => RepositoryResult::Ok(query_result),
    }
}

pub fn insert_user(new_user_dto: NewUserDTO) -> RepositoryResult<User, String> {
    let new_user: NewUser = new_user_dto.into();
    let new_users = vec![new_user];
    let connection = &mut get_connection();
    let results: Result<Vec<User>, diesel::result::Error> = diesel::insert_into(app_users)
        .values(&new_users)
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

pub fn get_by_email(email_string: String) -> RepositoryResult<User, String> {
    let connection = &mut get_connection();
    let result = app_users.filter(email.eq(email_string)).first::<User>(connection);
    match result {
        QueryResult::Err(error) => {
            error!("{}", error);
            return RepositoryResult::Err(String::from("Login failed"));
        }
        QueryResult::Ok(user) => {
            return RepositoryResult::Ok(user);
        }
    }
}

pub fn get_by_user_name(user_name_string: String) -> RepositoryResult<User, String> {
    let connection = &mut get_connection();
    let result = app_users.filter(user_name.eq(user_name_string)).first::<User>(connection);
    match result {
        QueryResult::Err(error) => {
            error!("{}", error);
            return RepositoryResult::Err(String::from("Login failed"));
        }
        QueryResult::Ok(user) => {
            return RepositoryResult::Ok(user);
        }
    }
}

pub fn get_user_by_id(user_id_data:i32) -> RepositoryResult<User, String> {
    let connection = &mut get_connection();
    let result = app_users
        .filter(id.eq(user_id_data))
        .first::<User>(connection);
    match result {
        QueryResult::Err(error) => {
            error!("{}", error);
            return RepositoryResult::Err(String::from("User not found"));
        }
        QueryResult::Ok(user) => {
            // if user.get(0).is_none() {
            //     return RepositoryResult::Err(String::from("User not found"));
            // }
            // return RepositoryResult::Ok(*user.get(0).unwrap());
            return RepositoryResult::Ok(user);
        }
    }
}
