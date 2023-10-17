use serde::{Deserialize, Serialize};
use crate::applications::general::USER;
use crate::auth::hash::hash_password;

use crate::repository::user::{NewUser, User};

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct UserDTO {
    pub id: i32,
    pub user_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct NewUserDTO {
    pub user_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct UserLoginDTO {
    pub user_name: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct LoginResponseDTO {
    pub role: String,
    pub access_token: String,
    pub refresh_token: String,
}

impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        UserDTO {
            id: user.id,
            user_name: user.user_name,
            email: user.email,
            password: user.password
        }
    }
}

impl From<NewUser> for NewUserDTO {
    fn from(new_user: NewUser) -> Self {
        NewUserDTO {
            user_name: new_user.user_name,
            email: new_user.email,
            password: new_user.password
        }
    }
}

impl From<NewUserDTO> for NewUser {
    fn from(new_user_dto: NewUserDTO) -> Self {
        NewUser {
            user_name: new_user_dto.user_name,
            email: new_user_dto.email,
            role: String::from(USER),
            password: hash_password(new_user_dto.password)
        }
    }
}