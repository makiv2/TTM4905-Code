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
        .mount("/", routes![index, api::health])
        .mount("/", routes![api::user_api::create_user, api::user_api::get_users, api::user_api::get_user, api::user_api::delete_user,
                            api::zk_api::generate_proof, api::zk_api::get_proofs, api::zk_api::get_proof, api::zk_api::verify])
        .configure(|rocket| {
            rocket
                .port(6666)
                .address("0.0.0.0")
        })
        // .manage(user_service)
}
