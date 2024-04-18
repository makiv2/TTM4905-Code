use argon2::{self, Config};
use reqwest::Error;
use serde::Deserialize;
use sha2::{Digest, Sha512};
use sp1_core::{SP1Prover, SP1Stdin};
use std::env;

#[derive(Deserialize, Debug)]
struct User {
    username: String,
    password: String,
}

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the required arguments are provided
    if args.len() < 4 {
        println!("Usage: {} <expected_username> <expected_password> <message>", args[0]);
        return Ok(());
    }

    // Extract the expected username and password from the command-line arguments
    let expected_username: String = args[1].clone();
    let expected_password: String = args[2].clone();
    let message: String = args[3].clone();

    // Hash the expected username using SHA-512
    let mut hasher = Sha512::new();
    hasher.update(expected_username.as_bytes());
    let expected_username_hash: String = format!("{:x}", hasher.finalize());

    // Hash the expected password using Argon2 with a salt
    let config: Config<'_> = Config::default();
    let salt: &[u8; 16] = b"some_random_salt";
    let expected_password_hash: String = argon2::hash_encoded(expected_password.as_bytes(), salt, &config).unwrap();

    // URL of the Django API endpoint for fetching users
    // Locally let url = "http://localhost:8000/api/users/";
    // DJANGO_HOST should be http://localhost:8000 locally or http://django-backend:8000 in the container

    let url: String;

    if let Ok(django_host) = env::var("DJANGO_HOST") {
        url = format!("{}/api/users/", django_host);
    } else {
        println!("DJANGO_HOST environment variable is not set.");
        return Ok(());
    }

    // Send a GET request to the API endpoint
    let response: Vec<User> = reqwest::get(&url)
        .await?
        .json::<Vec<User>>()
        .await?;

    // Dummy company name
    let company_name: String = "Netcompany".to_string();

    // Flag to track if a matching user is found
    let mut user_found: bool = false;

    // Iterate through the fetched users and test each one with the prover program
    for user in response {
        // Hash the test username using SHA-512
        let mut hasher = Sha512::new();
        hasher.update(user.username.as_bytes());
        let test_username_hash: String = format!("{:x}", hasher.finalize());

        // Hash the test password using Argon2 with the same salt
        let test_password_hash: String = argon2::hash_encoded(user.password.as_bytes(), salt, &config).unwrap();

        // Generate input for the current user
        let mut stdin: SP1Stdin = SP1Stdin::new();
        stdin.write(&expected_username_hash);
        stdin.write(&expected_password_hash);
        stdin.write(&test_username_hash);
        stdin.write(&test_password_hash);
        stdin.write(&company_name);
        stdin.write(&message);

        // Execute the ELF binary with the input
        let mut stdout: sp1_core::SP1Stdout = SP1Prover::execute(ELF, stdin).expect("execution failed");

        // Read output from the execution
        let output: String = stdout.read::<String>();

        // Parse the output string
        let output: serde_json::Value = serde_json::from_str(&output).expect("failed to parse output");

        // Check the value of the "match" field
        if output["match"].as_bool().unwrap_or(false) {
            println!("Matching user found:");
            println!("Username: {}, Password: {}", user.username, user.password);
            println!("Company: {}", output["company"].as_str().unwrap_or(""));
            println!("Message: {}", &message);
            user_found = true;

            // If user is found, generate the proof, start by creating new stdin
            let mut new_stdin = SP1Stdin::new(); // Create a new instance of SP1Stdi

            // Populate it
            new_stdin.write(&expected_username_hash);
            new_stdin.write(&expected_password_hash);
            new_stdin.write(&test_username_hash);
            new_stdin.write(&test_password_hash);
            new_stdin.write(&company_name);
            new_stdin.write(&message);

            let proof = SP1Prover::prove_only_output(ELF, new_stdin).expect("proving failed"); // Use the new instance of SP1Stdin
            let proof_filename = format!("credentials_check_proof.json");
            proof.save(&proof_filename).expect("saving proof failed");

            // Exit program after proof is generated
            break;
        }
    }

    if !user_found {
        println!("No matching user found in the database.");
    }

    Ok(())
}