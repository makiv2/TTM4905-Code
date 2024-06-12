use reqwest::Error;
use serde::Deserialize;
use sp1_core::{SP1Prover, SP1Stdin};
use std::env;
use base64;

#[derive(Deserialize, Debug)]
struct PubKey {
    key: String,
}

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

// String to bytes function
fn str_to_bytes(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Read the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <message_b64> <signature_b64> <company_b64>", args[0]);
        std::process::exit(1);
    }
    // New parameters are not filepaths, but the files b64 encoded

    let message_b64 = &args[1];
    let signature_b64 = &args[2];
    let company_name_b64 = &args[3];

    // Decode the b64 encoded files
    // Decode the mesaage
    let message: Vec<u8> = base64::decode(message_b64).expect("Error decoding message");
    // Define message as a string
    let message_str = String::from_utf8(message.clone()).expect("Invalid UTF-8 message");
  

    // Decode the signature
    let signature_bytes: Vec<u8> = base64::decode(signature_b64).expect("Error decoding signature");

    // Decode the company name
    let company_name: Vec<u8> = base64::decode(company_name_b64).expect("Error decoding company name");
    // Define company name as a string
    let company_name = String::from_utf8(company_name.clone()).expect("Invalid UTF-8 company name");

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
        println!("No compatible signature found in the database.");
    }

    Ok(())
}