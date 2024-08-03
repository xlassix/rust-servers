use bcrypt::BcryptError;
use rocket::serde::json::{json, Value};
use rocket::serde::Serialize;
use rocket::Responder;
use serde::Deserialize;

#[derive(Responder, Debug)]
pub enum NetworkResponse {
    #[response(status = 200)]
    StatusOk(Value),
    #[response(status = 201)]
    Created(Value),
    #[response(status = 400)]
    BadRequest(Value),
    #[response(status = 401)]
    Unauthorized(Value),
    #[response(status = 404)]
    NotFound(Value),
    #[response(status = 409)]
    Conflict(Value),
    #[response(status = 500)]
    InternalServerError(Value),
}

#[derive(Serialize, Deserialize)]
pub enum ResponseBody {
    Message(String),
    AuthToken(String),
    Body(Value),
    Error(Value)
}


#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    pub data: ResponseBody,
    pub message: String,

}


#[derive(Debug)]
pub enum CustomError {
    DatabaseError(diesel::result::Error),
    PasswordError(BcryptError),

}


impl From<diesel::result::Error> for CustomError {
    fn from(error: diesel::result::Error) -> Self {
        CustomError::DatabaseError(error)
    }
}

impl From<BcryptError> for CustomError {
    fn from(error: BcryptError) -> Self {
        CustomError::PasswordError(error)
    }
}


impl From<CustomError> for NetworkResponse {
    fn from(error: CustomError) -> Self {
        match error {
            CustomError::DatabaseError(err) => {
                let response = ErrorResponse {
                    data: ResponseBody::Error(format!("{:?}",err).into()),
                    message:"Database Error".to_string(),
                };
                NetworkResponse::BadRequest(json!(response))
            },
            CustomError::PasswordError(err) => {
                let response = ErrorResponse {
                    data: ResponseBody::Error(format!("{:?}",err).into()),
                    message:"Password Error".to_string(),
                };
                NetworkResponse::BadRequest(json!(response))
            }
        }
    }
}