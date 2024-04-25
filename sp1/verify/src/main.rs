use sp1_core::{SP1ProofOnlyOutput, SP1Verifier};
use sp1_core::utils::BabyBearBlake3;
use std::fs;
use serde_json;
use std::env;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file_path>", args[0]);
    }

    let json_filepath: String = args[1].clone();
    // Read the proof from the JSON file.
    let proof_json = fs::read_to_string(json_filepath).expect("failed to read proof file");
    let mut proof: SP1ProofOnlyOutput<BabyBearBlake3> =
        serde_json::from_str(&proof_json).expect("failed to deserialize proof");

    // Verify the proof.
    SP1Verifier::verify_only_output(ELF, &proof).expect("verification failed");

    // Read the output from the proof.
    let output_str = proof.stdout.read::<String>();

    // Parse the output string
    let output: serde_json::Value = serde_json::from_str(&output_str).expect("failed to parse output");

    // Extract the relevant fields from the output
    let credentials_match = output["match"].as_bool().unwrap_or(false);
    let company = output["company"].as_str().unwrap_or("");
    let message = output["message"].as_str().unwrap_or("");

    // Print the extracted information
    println!("Credentials match: {}", credentials_match);
    println!("Company: {}", company);
    println!("Message: {}", message);

    println!("Successfully verified the proof for the credentials check program!");
}