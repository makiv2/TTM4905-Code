use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};
use std::fs::File;
use std::io::{self, Read};
use std::env;
use serde_json;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn read_file_bytes(path: &str) -> Result<Vec<u8>, io::Error> {
    let mut file = File::open(path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    Ok(content)
}

fn main() {
    // Read the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} <public_key.pem> <message.txt> <message.txt.sig> <company>", args[0]);
        std::process::exit(1);
    }
    let pub_key_path = &args[1];
    let message_path = &args[2];
    let signature_path = &args[3];
    let company_name = &args[4];

    // Init stdin.
    let mut stdin = SP1Stdin::new();

    // Read and parse the public key
    let public_key_pem: Vec<u8>= read_file_bytes(pub_key_path).expect("Error reading public key file");
    // Print public key
    println!("Public key: {:?}", String::from_utf8(public_key_pem.clone()).expect("Invalid UTF-8 public key"));
    // Input public key
    stdin.write(&public_key_pem);

    // Read message to memory
    let message: Vec<u8> = read_file_bytes(message_path).expect("Error reading message file");
    // Print message
    println!("Message: {:?}", String::from_utf8(message.clone()).expect("Invalid UTF-8 message"));
    // Input message
    stdin.write(&message);

    // Read signature to memory
    let signature_bytes: Vec<u8> = read_file_bytes(signature_path).expect("Error reading signature file");
    // Input signature
    stdin.write(&signature_bytes);

    // Input company name
    stdin.write(&company_name);
    println!("All inputs written");
    
    let mut proof = SP1Prover::prove_only_output(ELF, stdin).expect("proving failed");
    println!("Proof generated");
    // Read output.
    let output_str: String = proof.stdout.read::<String>();

    // Parse the output string
    println!("Parsing output");
    let output: serde_json::Value = serde_json::from_str(&output_str).expect("failed to parse output");

    // Extract the relevant fields from the output
    let credentials_match = output["match"].as_bool().unwrap_or(false);
    let company = output["company"].as_str().unwrap_or("");
    let message = output["message"].as_str().unwrap_or("");

    // Print the extracted information
    println!("Credentials match: {}", credentials_match);
    println!("Company: {}", company);
    println!("Message: {}", message);
    
    // Verify proof.
    SP1Verifier::verify_only_output(ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("credentials_check_proof.json")
        .expect("saving proof failed");

    println!("Successfully generated and verified proof for the credentials check program!");
}