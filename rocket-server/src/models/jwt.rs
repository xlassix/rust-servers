use crate::utils::jwt::decode_jwt;
use crate::models::response::{NetworkResponse, Response, ResponseBody};
use jsonwebtoken::errors::Error;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::json;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub user_id: String,
    pub exp: u64,
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = NetworkResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, NetworkResponse> {
        fn is_valid(key: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(String::from(key))?)
        }

        match req.headers().get_one("authorization") {
            None => {
                let response = Response {
                    body: ResponseBody::Message(String::from(
                        "Error validating JWT token - No token provided",
                    )),
                };
                Outcome::Error((
                    Status::Unauthorized,
                    NetworkResponse::Unauthorized(json!(response)),
                ))
            }
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT { claims }),
                Err(err) => match &err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        let response = Response {
                            body: ResponseBody::Message(format!(
                                "Error validating JWT token - Expired Token"
                            )),
                        };
                        Outcome::Error((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(json!(response)),
                        ))
                    }
                    jsonwebtoken::errors::ErrorKind::InvalidToken => {
                        let response = Response {
                            body: ResponseBody::Message(format!(
                                "Error validating JWT token - Invalid Token"
                            )),
                        };
                        Outcome::Error((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(json!(response)),
                        ))
                    }
                    _ => {
                        let response = Response {
                            body: ResponseBody::Message(format!(
                                "Error validating JWT token - {}",
                                err
                            )),
                        };
                        Outcome::Error((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(json!(response)),
                        ))
                    }
                },
            },
        }
    }
}