mod api;
mod models;
mod repository;
mod schema;
mod services;

#[macro_use]
extern crate rocket;

use rocket::{get, post};
use rocket::serde::json::Json;
use serde::Deserialize;
use tokio::process::Command;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[derive(Deserialize)]
struct Credentials {
    company: String, // Dead code, but probably should be used somewhere?
    username: String,
    password: String,
}

#[get("/")]
fn index() -> &'static str {
    "Real shit, world!"
}

#[post("/check_credentials", data = "<credentials>")]
async fn check_credentials(credentials: Json<Credentials>) -> String {
    let username = &credentials.username;
    let password = &credentials.password;

    // Run the script executable
    let output = Command::new("cargo")
        .arg("run")
        .arg("--release")
        .arg("--manifest-path")
        .arg("/sp1/auxiliary/Cargo.toml")
        .arg(username)
        .arg(password)
        .output()
        .await
        .expect("failed to execute script");

    // Check if the execution was successful
    if output.status.success() {
        // Get the proof content from the script output
        let proof_content = String::from_utf8_lossy(&output.stdout);
        proof_content.to_string()
    } else {
        // Return an error response if the execution failed
        String::from("Error: Failed to execute the credentials check")
    }
}


#[launch]
fn rocket() -> _ {
    // Create any necessary dependencies for UserService
    // let user_repository = repository::UserRepository::new();
    // Create UserService with dependencies and manage it with rocket with .manage() this doesnt work but i dont think its worth fixing
    // let user_service = services::UserService::new(user_repository);

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allow_credentials(true);

    rocket::build()
        .mount("/", routes![index, check_credentials, api::health])
        .mount("/", routes![api::user_api::create_user, api::user_api::get_users, api::user_api::get_user, api::user_api::delete_user,
                            api::zk_api::generate_proof, api::zk_api::get_proofs, api::zk_api::get_proof, api::zk_api::verify])
        .attach(cors.to_cors().unwrap())
        // .manage(user_service)
}