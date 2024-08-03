use rocket::serde::json::{Json, Value};
use rocket:: {State};

use crate::models::db::*;
use crate::models::response::{NetworkResponse};

use crate::utils::db_connection::DBPool;
use crate::services::user_service;

#[post("/login",data = "<body>")]
pub async fn login(
    _db: &State<DBPool>,
    body: Json<AuthPayload<'_>>,
) -> Result<Json<Value>, NetworkResponse> {


    let post_request = body.into_inner();
    let post_request = post_request.validate()?;

    let response = user_service::login(_db,&post_request).await;

    match response {
        Ok(value) => Ok(Json(value)),
        Err(err) => Err(err),
    }
}


#[post("/register",data = "<body>")]
pub async fn register(
    _db: &State<DBPool>,
    body: Json<AuthPayload<'_>>,
) -> Result<NetworkResponse, NetworkResponse> {


    let post_request = body.into_inner();
    let post_request = post_request.validate()?;

    let response = user_service::register(_db,&post_request).await;

    match response {
        Ok(value) => Ok(NetworkResponse::Created(value)),
        Err(err) => Err(err),
    }
}
