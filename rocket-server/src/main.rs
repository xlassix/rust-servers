#[macro_use]
extern crate rocket;

pub mod utils;
pub mod schema;
pub mod services;
pub mod handler;
pub mod models;

use dotenvy::dotenv;
use handler::{catcher_handler::*, post_handler::*, user_handler::{login, register}};
use utils::db_connection::build_pool;

#[launch]
fn rocket() -> _ {
    dotenv().expect("Error occured when loading .env");

    rocket::build()
    .mount("/auth", routes![login,register])
    .mount("/post", routes![get_my_post,get_post_all,create_post,delete_post,update_post])
    .register("/", catchers![unprocessable_entity_catcher,not_found])
    .manage(build_pool())
}
