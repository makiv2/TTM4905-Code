use std::fs::File;
use std::io::{self, Read};
use std::env;
use ed25519_dalek::{pkcs8::DecodePublicKey, Signature, Verifier, VerifyingKey};
use pem::parse;

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn read_file_bytes(path: &str) -> Result<Vec<u8>, io::Error> {
    let mut file = File::open(path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    Ok(content)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <public_key.pem> <message.txt> <message.txt.sig>", args[0]);
        std::process::exit(1);
    }
    let pub_key_path = &args[1];
    let message_path = &args[2];
    let signature_path = &args[3];
    
    // Read and parse the public key
    let public_key_pem = read_file(pub_key_path).map_err(|e| {
        format!("Error reading public key file '{}': {}", pub_key_path, e)
    })?;
    let public_key_pem = parse(public_key_pem).map_err(|e| {
        format!("Error parsing PEM file '{}': {}", pub_key_path, e)
    })?;
    let public_key_der = public_key_pem.contents;

    // Decode the public key
    let verifying_key = VerifyingKey::from_public_key_der(&public_key_der).map_err(|e| {
        format!("Error decoding public key DER '{}': {}", pub_key_path, e)
    })?;

    // Read the message
    let message = read_file_bytes(message_path).map_err(|e| {
        format!("Error reading message file '{}': {}", message_path, e)
    })?;

    // Message in human-readable format
    println!("Message: {:?}", String::from_utf8(message.clone()).map_err(|e| {
        format!("Error: Message contains invalid UTF-8: {}", e)
    })?.trim());

    // Read the signature
    let signature_bytes = read_file_bytes(signature_path).map_err(|e| {
        format!("Error reading signature file '{}': {}", signature_path, e)
    })?;

    // Decode the signature
    let signature = Signature::try_from(&signature_bytes[..]).map_err(|e| {
        format!("Error decoding signature format '{}': {}", signature_path, e)
    })?;

    // Verify the signature
    match verifying_key.verify(&message, &signature) {
        Ok(_) => println!("Signature is valid!"),
        Err(e) => eprintln!("Signature is invalid: {}", e),
    }

    Ok(())
}
