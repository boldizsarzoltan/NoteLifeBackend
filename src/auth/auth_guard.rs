use rocket::http::Status;
use rocket::request::Outcome;
use rocket::Request;
use rocket::request::FromRequest;

#[derive(Debug)]
pub struct AuthenticatedUser {
    user_hash: String
}

#[derive(Debug)]
pub struct AuthenticatedAdmin {
    user_hash: String
}

#[derive(Debug)]
pub enum AuthError {
    InvalidRequest,
    InvalidHash,
    UsernameDoesNotHaveAccess,
    UsernameEmailPasswordInvalid
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for AuthenticatedUser {
    type Error = AuthError;
    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        println!("{:?}", request);
        if request.headers().contains("Authorization") {
            let auth_user = AuthenticatedUser {
                user_hash: String::from("hash")
            };
            return Outcome::Success(auth_user);
        }
        return Outcome::Failure((Status::Unauthorized, AuthError::InvalidRequest));
    }
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for AuthenticatedAdmin {
    type Error = AuthError;
    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        println!("{:?}", request);
        if request.headers().contains("Authorization") {
            let auth_user = AuthenticatedAdmin {
                user_hash: String::from("hash")
            };
            return Outcome::Success(auth_user);
        }
        return Outcome::Failure((Status::Unauthorized, AuthError::InvalidRequest));
    }
}