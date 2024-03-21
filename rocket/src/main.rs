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


#[get("/generate")]
fn generate() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from rust!")))
}


#[get("/verify")]
fn verify() -> &'static str {
    "Hello, world!"
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, generate, verify])
        .mount("/", routes![api::create_user, api::get_users])
}

