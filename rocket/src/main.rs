mod api;
mod models;
mod repository;
mod schema;
mod services;

#[macro_use]
extern crate rocket;

use rocket::{get};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    // Create any necessary dependencies for UserService
    // let user_repository = repository::UserRepository::new();
    // Create UserService with dependencies and manage it with rocket with .manage() this doesnt work but i dont think its worth fixing
    // let user_service = services::UserService::new(user_repository);

    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![api::create_user, api::get_users, api::get_user, api::delete_user,
                            api::generate, api::verify])
}
