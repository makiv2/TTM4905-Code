use reqwest::Error;
use serde::Deserialize;
use sp1_core::{SP1Prover, SP1Stdin, SP1Stdout};
use std::fs;

#[derive(Deserialize, Debug)]
struct User {
    id: i32,
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

    // Iterate through the fetched users and test each one with the prover program
    for user in response {
        // Generate input for the current user
        let mut stdin = SP1Stdin::new();
        let username: String = user.username.clone();
        let password: String = user.password.clone();
        stdin.write(&username);
        stdin.write(&password);

        // Execute the ELF binary with the input
        let mut stdout = SP1Prover::execute(ELF, stdin).expect("execution failed");

        // Read output from the execution
        let credentials_match = stdout.read::<bool>();

        if credentials_match {
            println!("Matching user found:");
            println!("ID: {}, Username: {}, Password: {}", user.id, user.username, user.password);
            break;
        }
    }

    Ok(())
}