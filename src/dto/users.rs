use crate::repository::user::{NewUser, User, LoginResponse};
use crate::auth::hash::{hash_password};
use serde::{Serialize, Deserialize};

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
    pub token: String,
}

impl From<LoginResponse> for LoginResponseDTO{
    fn from(login_response: LoginResponse) -> Self {
        LoginResponseDTO {
            role: login_response.role,
            token: login_response.token
        }
    }
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

impl From<UserDTO> for User{
    fn from(user: UserDTO) -> Self {
        User {
            id: user.id,
            user_name: user.user_name,
            email: user.email,
            password: user.password
        }
    }
}

impl From<NewUserDTO> for NewUser{
    fn from(new_user: NewUserDTO) -> Self {
        NewUser {
            user_name: new_user.user_name,
            email: new_user.email,
            password: hash_password(new_user.password)
        }
    }
}