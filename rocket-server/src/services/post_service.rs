use crate::utils::db_connection::{get_conn, DBPool};

use crate::models::db::{NewPost, NewPostDto, Post, UpdatePostDto};
use crate::models::response::{CustomError, NetworkResponse};

use rocket::serde::json::{json, Json, Value};
use rocket::State;

use crate::schema::posts;
use crate::schema::posts::dsl::*;

use diesel::prelude::*;

pub async fn get_posts(_db: &State<DBPool>) -> Result<Value, NetworkResponse> {
    let mut conn = get_conn(_db)?;

    let result = posts::table
        .select(Post::as_select())
        .load(&mut conn)
        .map_err(CustomError::from);

    match result {
        Ok(res) => Ok(json!(res)),
        Err(err) => Err(NetworkResponse::from(err)),
    }
}

pub async fn create_post(
    _db: &State<DBPool>,
    post_request: &NewPostDto<'_>,
) -> Result<Value, NetworkResponse> {
    let mut conn = get_conn(_db)?;

    if post_request.title.is_empty() {
        return Err(NetworkResponse::BadRequest("Title cannot be empty".into()));
    }
    let cuid = cuid::cuid2().to_string();

    let new_post = NewPost {
        id: &cuid,
        title: post_request.title,
        body: post_request.body,
        user_id:"",
    };

    let result: Result<Post, CustomError> = diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(&mut conn)
        .map_err(CustomError::from);

    match result {
        Ok(post) => Ok(json!(post)),
        Err(err) => Err(NetworkResponse::from(err)),
    }
}

pub async fn delete_post(_db: &State<DBPool>, post_id: &str) -> Result<Value, NetworkResponse> {
    let mut conn = get_conn(_db)?;


    let result = diesel::delete(posts)
        .filter(id.eq(post_id))
        .returning(Post::as_returning())
        .get_result(&mut conn)
        .map_err(CustomError::from);

    match result {
        Ok(post) => Ok(json!(post)),
        Err(err) => Err(NetworkResponse::from(err)),
    }
}

pub async fn update_post(
    _db: &State<DBPool>,
    put_request: Json<UpdatePostDto<'_>>,
) -> Result<Value, NetworkResponse> {
    let mut conn = get_conn(_db)?;

    let result = diesel::update(posts.find(put_request.id))
        .set((title.eq(put_request.title), body.eq(put_request.body)))
        .returning(Post::as_returning())
        .get_result(&mut conn)
        .map_err(CustomError::from);

    match result {
        Ok(post) => Ok(json!(post)),
        Err(err) => Err(NetworkResponse::from(err)),
    }
}