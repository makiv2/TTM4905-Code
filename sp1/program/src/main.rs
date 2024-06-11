#![no_main]
use ed25519_dalek::{pkcs8::DecodePublicKey, Signature, Verifier, VerifyingKey};
use pem::parse;


sp1_zkvm::entrypoint!(main);

pub fn main() {
    // Read the key files
    let pub_key = sp1_zkvm::io::read::<Vec<u8>>();
    let message = sp1_zkvm::io::read::<Vec<u8>>();
    let signature = sp1_zkvm::io::read::<Vec<u8>>();

    // Read the company name the user belongs to
    let company_name = sp1_zkvm::io::read::<String>();

    // Parse the public key
    let public_key_pem = parse(pub_key).expect("Failed to parse public key");
    let public_key_der = public_key_pem.contents;

    let verifying_key = VerifyingKey::from_public_key_der(&public_key_der)
        .expect("Invalid public key DER");

    // Human readable message content
    let message_content = String::from_utf8(message.clone()).expect("Invalid UTF-8 message");

    // Parse signature
    let signature = Signature::try_from(&signature[..])
        .expect("Invalid signature format");

    let mut signature_match: bool = false;
    match verifying_key.verify(&message, &signature) {
        Ok(_) => signature_match = true,
        Err(_) => signature_match = false,
    }
    // Create the output string with the message included
    let output = format!("{{\"match\": {}, \"company\": \"{}\", \"message\": \"{}\"}}", signature_match, company_name, message_content);

    // Write the result
    sp1_zkvm::io::write(&output);
}