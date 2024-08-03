use rocket::serde::json::{ Json, Value};
use rocket::State;

use crate::models::db::{NewPostDto, UpdatePostDto};
use crate::models::jwt::JWT;  
use crate::models::response::NetworkResponse;

use crate::utils::db_connection::DBPool;
use crate::services::post_service;

#[get("/")]
pub async fn get_my_post(
    _db: &State<DBPool>,
    key: Result<JWT, NetworkResponse>,
) -> Result<Value, NetworkResponse> {
    let _jwt = key?;

    let response = post_service::get_my_posts(_db,&_jwt.claims.user_id).await;

    match response {
        Ok(value) => Ok(value),
        Err(err) => Err(err),
    }
}


#[get("/all")]
pub async fn get_post_all(
    _db: &State<DBPool>,
    key: Result<JWT, NetworkResponse>,
) -> Result<Value, NetworkResponse> {
    let _jwt = key?;

    let response = post_service::get_posts_all(_db).await;

    match response {
        Ok(value) => Ok(value),
        Err(err) => Err(err),
    }
}

#[post("/", data = "<body>")]
pub async fn create_post(
    _db: &State<DBPool>,
    body: Json<NewPostDto<'_>>,
    key: Result<JWT, NetworkResponse>,
) -> Result<Value, NetworkResponse> {
    let _jwt = key?;

    let post_request = body.into_inner();
    let post_request = post_request.validate()?;

    let response = post_service::create_post(_db, &post_request,&_jwt.claims.user_id).await;

    match response {
        Ok(value) => Ok(value),
        Err(err) => Err(err),
    }
}

#[delete("/<post_id>")]
pub async fn delete_post(
    _db: &State<DBPool>,
    post_id: &str,
    key: Result<JWT, NetworkResponse>,
) -> Result<Value, NetworkResponse> {
    let _jwt = key?;

    let response = post_service::delete_post(_db, post_id).await;

    match response {
        Ok(value) => Ok(value),
        Err(err) => Err(err),
    }
}

#[put("/", data = "<put_request>")]
pub async fn update_post(
    _db: &State<DBPool>,
    put_request: Json<UpdatePostDto<'_>>,
    key: Result<JWT, NetworkResponse>,
) -> Result<Value, NetworkResponse> {
    let _jwt = key?;

    let response = post_service::update_post(_db, put_request).await;

    match response {
        Ok(value) => Ok(value),
        Err(err) => Err(err),
    }
}
