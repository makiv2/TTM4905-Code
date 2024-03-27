mod api;
mod models;
mod repository;
mod schema;
mod services;


#[macro_use]
extern crate rocket;

use rocket::{get, http::Status, serde::json::Json};


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![api::create_user, api::get_users, api::get_user, api::delete_user,
                            api::generate, api::verify])
}

