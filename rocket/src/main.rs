mod api;
mod models;
mod repository;
mod schema;
mod services;

#[macro_use]
extern crate rocket;

use rocket::{get};
use rocket_cors::{AllowedOrigins, CorsOptions};


#[get("/")]
fn index() -> &'static str {
    "Real shit, world!"
}

#[launch]
fn rocket() -> _ {
    // Create any necessary dependencies for UserService
    // let user_repository = repository::UserRepository::new();
    // Create UserService with dependencies and manage it with rocket with .manage() this doesnt work but I do not think its worth fixing
    // let user_service = services::UserService::new(user_repository);

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allow_credentials(true);

    rocket::build()
        .mount("/", routes![index, api::health])
        .mount("/", routes![api::user_api::create_user, api::user_api::get_users, api::user_api::get_user, api::user_api::delete_user,
                            api::zk_api::generate_proof, api::zk_api::get_proofs, api::zk_api::get_proof, api::zk_api::get_raw_proof, api::zk_api::verify])
        .attach(cors.to_cors().unwrap())
        // .manage(user_service)
}