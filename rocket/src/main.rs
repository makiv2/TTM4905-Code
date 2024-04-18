mod api;
mod models;
mod repository;
mod schema;
mod services;

#[macro_use]
extern crate rocket;

use std::fs;
use std::io::stdout;

use rocket::{get, post};
use rocket::serde::json::{self, Json};
use serde::Deserialize;
use serde_json::Value;
use serde_json::from_str;
use tokio::fs::File;
use tokio::process::Command;
use rocket::tokio::io::AsyncWriteExt;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[derive(Deserialize)]
struct Credentials {
    company: String, // Dead code, but probably should be used somewhere?
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct GeneratedProof {
    verified: bool,
    company: String,
    message: String,
}

#[get("/")]
fn index() -> &'static str {
    "Real shit, world!"
}

#[post("/check_credentials", data = "<credentials>")]
async fn check_credentials(credentials: Json<Credentials>) -> String {
    let username = &credentials.username;
    let password = &credentials.password;
    let company = &credentials.company;

    // Run the script executable
    let output = Command::new("../sp1/auxiliary/target/release/auxiliary")
        .arg(username)
        .arg(password)
        .arg(company)
        .output()
        .await
        .expect("failed to execute script");

    // Check if the execution was successful
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("This is the stdout {}", stdout);

    // Check if the execution was successful
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("This is the stderr {}", stderr);

    if output.status.success() {
        // Get the proof content from the script output
        let proof_content = String::from_utf8_lossy(&output.stdout);
        
        // Check if the output is equal to "No matching user found in the database."
        if proof_content.trim() == "No matching user found in the database." {
            String::from("Failure")
        } else {
            // Read json file created
        let json_file_path = "credentials_check_proof.json";
        let json_content = fs::read_to_string(json_file_path).expect("Failed to read JSON file");

        // Parse the JSON content
        let json: Value = serde_json::from_str(&json_content).expect("Failed to parse JSON");

        // Extract the proof from the JSON
        let proof = json["proof"].as_str().expect("Proof not found in JSON").to_string();

        // Extract the output from the JSON
        let output_data = json["stdout"]["buffer"]["data"].as_array().expect("Output data not found in JSON");
        let mut output_string = String::from_utf8(output_data.iter().map(|v| v.as_u64().unwrap() as u8).collect()).expect("Failed to convert output to string");
        output_string.remove(0).to_string();
        output_string.trim().to_string();
        
        // The output string is malformed json so we won't use it further but it should be possible
        println!("{:?}", output_string);

        // Parse the output string
        println!("Do I fail here?");
        println!("Output String: {}", output_string);
        // let unescaped_output = output_string.replace("\\\"", "\"");
        // let output: Value = serde_json::from_str(&unescaped_output).expect("Failed to parse output string");

        // Extract the match, company, and message from the output
        // let credentials_match = output["match"].as_bool().unwrap_or(false);
        // let company = output["company"].as_str().unwrap_or("").to_string();
        // let message = output["message"].as_str().unwrap_or("").to_string();
        // String::from("Company: ") + &company + &format!(", Match: {}, Message: {}", credentials_match, message)

        //Try uploading json
        let new_filename = "test_proof.json";
        json_content.stream_to_file(new_filename).await.expect("Failed to write to file");

        String::from("We did that shit")
        }
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