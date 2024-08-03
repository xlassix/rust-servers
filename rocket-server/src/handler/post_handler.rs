use rocket::serde::json::{Value};
use rocket::State;

use crate::models::jwt::JWT;  
use crate::models::response::NetworkResponse;

use crate::utils::db_connection::DBPool;
use crate::services::post_service;

#[get("/")]
pub async fn get_post(
    _db: &State<DBPool>,
    key: Result<JWT, NetworkResponse>,
) -> Result<Value, NetworkResponse> {
    let _jwt = key?;

    let response = post_service::get_posts(_db).await;

    match response {
        Ok(value) => Ok(value),
        Err(err) => Err(err),
    }
}
