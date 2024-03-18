use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    let username_hash = 123456789u128; // Example hash for "user123"
    let password_hash = 987654321u128; // Example hash for "pass123"
    stdin.write(&username_hash);
    stdin.write(&password_hash);
    let mut proof = SP1Prover::prove_only_output(ELF, stdin).expect("proving failed");

    // Read output.
    let credentials_match = proof.stdout.read::<bool>();
    println!("Credentials match: {}", credentials_match);

    // Verify proof.
    SP1Verifier::verify_only_output(ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("credentials_check_proof.json")
        .expect("saving proof failed");

    println!("Successfully generated and verified proof for the credentials check program!");
}