use sp1_core::{SP1ProofOnlyOutput, SP1Verifier};
use sp1_core::utils::BabyBearBlake3;
use std::fs;
use serde_json;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Read the proof from the JSON file.
    let proof_json = fs::read_to_string("credentials_check_proof.json").expect("failed to read proof file");
    let mut proof: SP1ProofOnlyOutput<BabyBearBlake3> =
        serde_json::from_str(&proof_json).expect("failed to deserialize proof");

    // Verify the proof.
    SP1Verifier::verify_only_output(ELF, &proof).expect("verification failed");

    // Read the output from the proof.
    let credentials_match = proof.stdout.read::<bool>();
    println!("Credentials match: {}", credentials_match);

    println!("Successfully verified the proof for the credentials check program!");
}