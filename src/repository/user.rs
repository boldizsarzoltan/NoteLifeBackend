use crate::repository::connection::get_connection;
use crate::schema::app_users as users_table;
use crate::schema::app_users::dsl::*;
use diesel::prelude::{QueryResult, Selectable, Insertable};
use crate::repository::types::{RepositoryResult};
use crate::auth::hash::{hash_password, verify_password};
use diesel::prelude::*;
use crate::dto::users::{NewUserDTO};
use diesel::RunQueryDsl;
use bcrypt::{BcryptResult};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = users_table)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub email: String,
    pub password: String,
}


#[derive(Insertable, Debug)]
#[diesel(table_name = users_table)]
pub struct NewUser {
    pub user_name: String,
    pub email: String,
    pub password: String,
}

pub struct LoginResponse {
    pub role: String,
    pub token: String,
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

pub fn login_by_email(email_string:String, password_data:String) -> RepositoryResult<LoginResponse, String> {
    let connection = &mut get_connection();
    let result = app_users.filter(email.eq(email_string)).first::<User>(connection);
    match result {
        QueryResult::Err(error) => {
            error!("{}", error);
            return RepositoryResult::Err(String::from("Login failed"));
        }
        QueryResult::Ok(user) => {
            return verify_password_local(user, password_data);
        }
    }
}

pub fn login_by_user_name(user_name_string:String, password_data:String) -> RepositoryResult<LoginResponse, String> {
    let connection = &mut get_connection();
    let result = app_users.filter(user_name.eq(user_name_string)).first::<User>(connection);
    match result {
        QueryResult::Err(error) => {
            error!("{}", error);
            return RepositoryResult::Err(String::from("Login failed"));
        }
        QueryResult::Ok(user) => {
            return verify_password_local(user, password_data);
        }
    }
}

pub fn verify_password_local(user:User, password_data:String) -> RepositoryResult<LoginResponse, String> {
    let result = verify_password(password_data, user.password.as_str());
    match result {
        BcryptResult::Err(error) => {
            error!("{}", error);
            return RepositoryResult::Err(String::from("Login failed"));
        }
        BcryptResult::Ok(user) => {
            if(user) {
                let login_response = LoginResponse {
                    role: String::from("role"),
                    token: String::from("token")
                };
                return RepositoryResult::Ok(login_response);
            }
            return RepositoryResult::Err(String::from("Login failed"));
        }
    }
}