use chrono;
use diesel::{ prelude::*};
use rocket::{serde::{json, Deserialize, Serialize}};

use super::response::{NetworkResponse, Response, ResponseBody};

#[derive(Queryable, Selectable, Serialize,Deserialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Post {
    pub id: String, // Changed from i32 to String to match the schema
    pub title: Option<String>, // Nullable<Text> becomes Option<String>
    pub body: String,
    pub user_id: String, // Changed from i32 to String to match the schema
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>, // Nullable<Timestamp> becomes Option<chrono::NaiveDateTime>
}

#[derive(Queryable, Selectable, Serialize, Deserialize,AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: String, // Changed from i32 to String to match the schema
    pub name: Option<String>, // Nullable<Text> becomes Option<String>
    pub username: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>, // Nullable<Timestamp> becomes Option<chrono::NaiveDateTime>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct AuthPayload<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

impl<'a> AuthPayload<'a> {
    pub fn validate(&self) -> Result<&AuthPayload<'a>, NetworkResponse> {
        if self.username.is_empty() {
            let response = Response {
                body: ResponseBody::Message("email cannot be empty".into()),
            };
            return Err(NetworkResponse::BadRequest(json::json!(response)));
        }


        if self.password.is_empty() {
            let response = Response {
                body: ResponseBody::Message("password cannot be empty".into()),
            };
            return Err(NetworkResponse::BadRequest(json::json!(response)));
        }

        if self.password.len()<6 {
            let response = Response {
                body: ResponseBody::Message("password cannot less than 6 characters long".into()),
            };
            return Err(NetworkResponse::BadRequest(json::json!(response)));
        }

        Ok(self)
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub name: &'a str,

}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub body: &'a str,
    pub user_id : &'a str
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NewPostDto<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UpdatePostDto<'a> {
    pub id:  &'a str,
    pub title: &'a str,
    pub body: &'a str,
}

impl<'a> NewPostDto<'a> {
    pub fn validate(&self) -> Result<&NewPostDto<'a>, NetworkResponse> {
        if self.title.is_empty() {
            let response = Response {
                body: ResponseBody::Message("Title cannot be empty".into()),
            };
            return Err(NetworkResponse::BadRequest(json::json!(response)));
        }

        if self.body.is_empty() {
            let response = Response {
                body: ResponseBody::Message("Body cannot be empty".into()),
            };
            return Err(NetworkResponse::BadRequest(json::json!(response)));
        }

        Ok(self)
    }
}
