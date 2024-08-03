#[macro_use]
extern crate rocket;

pub mod utils;
pub mod schema;
pub mod services;
pub mod handler;
pub mod models;

use dotenvy::dotenv;
use handler::{post_handler::*, user_handler::{login, register}};
use utils::db_connection::build_pool;

#[launch]
fn rocket() -> _ {
    dotenv().expect("Error occured when loading .env");

    rocket::build()
    .mount("/auth", routes![login,register])
    .mount("/post", routes![get_post])
    .manage(build_pool())
}
