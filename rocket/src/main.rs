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

    // Create any necessary dependencies for UserService
    let user_repository = repository::UserRepository::new();

    // Create UserService with dependencies
    let user_service = services::UserService::new(user_repository);

    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![api::create_user, api::get_users, api::get_user, api::delete_user,
                            api::generate, api::verify])
        .manage(user_service)
}

