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
    let public_key_pem = parse(pub_key.clone()).expect("Failed to parse public key");
    let public_key_der = public_key_pem.contents;
    let verifying_key = VerifyingKey::from_public_key_der(&public_key_der).expect("Failed to decode public key");

    // Make public key into a readable format
    let public_key_content = String::from_utf8(pub_key.clone()).expect("Faen");;

    // Read the message
    let message_content = String::from_utf8(message.clone()).expect("Failed to read message");
    // Trim the message
    let message_content = message_content.trim();
    
    // Read the signature
    let signature = Signature::try_from(&signature[..]).expect("Failed to decode signature");
    
    let valid_signature: bool;
    // Verify signature and return it as valid_signature
    match verifying_key.verify_strict(&message, &signature) {
        Ok(_) => valid_signature = true,
        Err(_) => valid_signature = false,
    };

    // Create the output string with the message included
    let output = format!("{{\"match\": {}, \"company\": \"{}\", \"message\": \"{}\"}}", valid_signature, company_name, message_content.to_string());

    // Write the result
    sp1_zkvm::io::write(&output);

}