use reqwest::Error;
use serde::Deserialize;
use sp1_core::{SP1Prover, SP1Stdin};
use std::env;
use std::fs::File;
use std::io::{self, Read};

#[derive(Deserialize, Debug)]
struct PubKey {
    key: String,
}

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn read_file_bytes(path: &str) -> Result<Vec<u8>, io::Error> {
    let mut file = File::open(path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    Ok(content)
}

// String to bytes function
fn str_to_bytes(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Read the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <message.txt> <message.txt.sig> <company>", args[0]);
        std::process::exit(1);
    }
    let message_path = &args[1];
    let signature_path = &args[2];
    let company_name = &args[3];

    // Read the message from file
    let message: Vec<u8> = read_file_bytes(message_path).expect("Error reading message file");
    // Define message as a string
    let message_str = String::from_utf8(message.clone()).expect("Invalid UTF-8 message");
    // Print message
    println!("Message: {:?}", message_str);

    // Read the signature from file
    let signature_bytes: Vec<u8> = read_file_bytes(signature_path).expect("Error reading signature file");

    // URL of the Django API endpoint for fetching users
    // Locally let url = "http://localhost:8000/api/users/";
    // DJANGO_HOST should be http://localhost:8000 locally or http://django-backend:8000 in the container

    let url: String;

    if let Ok(django_host) = env::var("DJANGO_HOST") {
        url = format!("{}/api/pubkeys/", django_host);
    } else {
        println!("DJANGO_HOST environment variable is not set.");
        return Ok(());
    }

    // Send a GET request to the API endpoint
    let response: Vec<PubKey> = reqwest::get(&url)
        .await?
        .json::<Vec<PubKey>>()
        .await?;

    // Flag to track if a matching user is found
    let mut user_found: bool = false;

    // Iterate through the fetched users and test each one with the prover program
    for pubkey in response {
        // Make the pubkey into the correct type
        let pubkey = pubkey.key;
        let pubkey_bytes = str_to_bytes(&pubkey);
        // Print public key from pem format
        println!("Public key: {:?}", String::from_utf8(pubkey_bytes.clone()).expect("Invalid UTF-8 public key"));

        // Generate input for the current pubkey
        let mut stdin: SP1Stdin = SP1Stdin::new();
        stdin.write(&pubkey_bytes.clone());
        stdin.write(&message.clone());
        stdin.write(&signature_bytes.clone());
        stdin.write(&company_name.clone());


        // Execute the ELF binary with the input
        let mut stdout: sp1_core::SP1Stdout = SP1Prover::execute(ELF, stdin).expect("execution failed");

        // Read output from the execution
        let output: String = stdout.read::<String>();

        // Parse the output string
        let output: serde_json::Value = serde_json::from_str(&output).expect("failed to parse output");

        // Check the value of the "match" field
        if output["match"].as_bool().unwrap_or(false) {
            println!("Signature was valid:");
            println!("Company: {}", output["company"].as_str().unwrap_or(""));
            println!("Message: {}", &message_str);
            user_found = true;

            // If user is found, generate the proof, start by creating new stdin
            let mut new_stdin = SP1Stdin::new(); // Create a new instance of SP1Stdin

            // Populate it
            new_stdin.write(&pubkey_bytes.clone());
            new_stdin.write(&message.clone());
            new_stdin.write(&signature_bytes.clone());
            new_stdin.write(&company_name.clone());

            let proof = SP1Prover::prove_only_output(ELF, new_stdin).expect("proving failed"); // Use the new instance of SP1Stdin
            let proof_filename = format!("credentials_check_proof.json");
            proof.save(&proof_filename).expect("saving proof failed");

            // // Exit program after proof is generated
            break;
        }
    }

    if !user_found {
        println!("No matching user found in the database.");
    }

    Ok(())
}