use reqwest::Error;
use serde::Deserialize;
use sp1_core::{SP1Prover, SP1Stdin};

#[derive(Deserialize, Debug)]
struct User {
    username: String,
    password: String,
}

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

#[tokio::main]
async fn main() -> Result<(), Error> {
    // URL of the Django API endpoint for fetching users
    let url = "http://localhost:8000/api/users/";

    // Send a GET request to the API endpoint
    let response = reqwest::get(url)
        .await?
        .json::<Vec<User>>()
        .await?;

    // Static expected user
    let expected_username: String = "tred".to_string();
    let expected_password: String = "bull".to_string();

    // Dummy company name
    let company_name: String = "Netcompany".to_string();

    // Flag to track if a matching user is found
    let mut user_found: bool = false;

    // Iterate through the fetched users and test each one with the prover program
    for user in response {
        // Generate input for the current user
        let mut stdin = SP1Stdin::new();
        stdin.write(&expected_username);
        stdin.write(&expected_password);
        let test_username: String = user.username.clone();
        let test_password: String = user.password.clone();
        stdin.write(&test_username);
        stdin.write(&test_password);
        stdin.write(&company_name);

        // Execute the ELF binary with the input
        let mut stdout = SP1Prover::execute(ELF, stdin).expect("execution failed");

        // Read output from the execution
        let output = stdout.read::<String>();

        // Parse the output string
        let output: serde_json::Value = serde_json::from_str(&output).expect("failed to parse output");

        // Check the value of the "match" field
        if output["match"].as_bool().unwrap_or(false) {
            println!("Matching user found:");
            println!("Username: {}, Password: {}", user.username, user.password);
            println!("Company: {}", output["company"].as_str().unwrap_or(""));
            user_found = true;
            break;
        }
    }
    if !user_found {
        println!("No matching user found in the database.");
    }

    Ok(())
}