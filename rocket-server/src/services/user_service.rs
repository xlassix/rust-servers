use crate::utils::db_connection::{get_conn, DBPool};

use crate::utils::jwt::{create_jwt};
use crate::models::db::{AuthPayload, NewUser, User};
use crate::models::response::{CustomError, ErrorResponse, NetworkResponse, Response, ResponseBody};

use bcrypt::{hash,verify,DEFAULT_COST};
use rocket::serde::json::{ json,  Value};
use rocket::State;

use crate::schema::users;
use crate::schema::users::dsl::*;

use diesel::prelude::*;


fn hash_password(_password: &str) -> Result<String, bcrypt::BcryptError> {
    let hashed_password = hash(_password, DEFAULT_COST)?;
    Ok(hashed_password)
}

pub async fn register(
    _db: &State<DBPool>,
    post_request: &AuthPayload<'_>,
) -> Result<Value, NetworkResponse> {
    let mut conn = get_conn(_db)?;

    if post_request.username.is_empty() {
        return Err(NetworkResponse::BadRequest("username cannot be empty".into()));
    }

    if post_request.password.is_empty() {
        return Err(NetworkResponse::BadRequest("password cannot be empty".into()));
    }

    if post_request.username.contains(" ") {
        return Err(NetworkResponse::BadRequest("username cannot contain white space".into()));
    }
    let normalized_username=post_request.username.to_lowercase();

    let user:Option<User>  = users::table
        .filter(username.eq(&normalized_username))
        .first(&mut conn)
        .optional().unwrap();

        match user {
            Some(_) => {
                let response =ErrorResponse{
                    data: ResponseBody::Message("A user already exist with this username".to_string()),
                    message:"Failed To create User".to_string()
                };
                return  Err(NetworkResponse::BadRequest(json!(response)));
            },
            None => {
                // If the user does not exist, proceed with other code.
            
            
                let hashed_password=hash_password(post_request.password).map_err(CustomError::from)?;
            
                let cuid = cuid::cuid2().to_string();
                let new_user = NewUser {
                    id:&cuid,
                    password: &hashed_password,
                    username: &normalized_username,
                    name:""
                };
            
                let result: Result<User, CustomError> = diesel::insert_into(users::table)
                    .values(&new_user)
                    .get_result(&mut conn)
                    .map_err(CustomError::from);
            
                match result {
                    Ok(_) => {
                        let response =Response{
                            body: ResponseBody::Message("User successfully created".to_string())
                        };
                        Ok(json!(response))},
                    Err(err) =>{ Err(NetworkResponse::from(err))},
                }
                // Place the code you want to execute here.
            }
        }
    
}


pub async fn login(
    _db: &State<DBPool>,
    post_request: &AuthPayload<'_>,
) -> Result<Value, NetworkResponse> {
    let mut conn = get_conn(_db)?;

    if post_request.username.is_empty() {
        return Err(NetworkResponse::BadRequest("username cannot be empty".into()));
    }

    if post_request.password.is_empty() {
        return Err(NetworkResponse::BadRequest("password cannot be empty".into()));
    }

    if post_request.username.contains(" ") {
        return Err(NetworkResponse::BadRequest("username cannot contain white space".into()));
    }
    let normalized_username=post_request.username.to_lowercase();

    let user:Option<User>  = users::table
        .filter(username.eq(&normalized_username))
        .first(&mut conn)
        .optional().unwrap();

        match user { 
            Some(user) => {
                let is_verified=verify(post_request.password,&user.password).map_err(CustomError::from)?;
                if!is_verified{
                    let response = ErrorResponse {
                        data: ResponseBody::Body("Invalid userName or Password".into()),
                        message:"Invalid Credential".to_string(),
                    };
                    return Err(NetworkResponse::BadRequest(json!(response)));
                }
                let jwt=create_jwt(user.id);
                match jwt {
                    Ok(jwt)=>{
                        let response = Response {
                            body: ResponseBody::AuthToken(jwt),
                            
                        };
                        Ok(json!(response))
                    },
                    Err(_)=>  Err(NetworkResponse::InternalServerError("Could Not generate JWT Token".into())),

                }
            },
            None => {
                let response = ErrorResponse {
                    data: ResponseBody::Body("Invalid userName or Password".into()),
                    message:"Invalid Credential".to_string(),
                };
                return Err(NetworkResponse::BadRequest(json!(response)));
            }
        }
    
}