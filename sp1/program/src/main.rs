#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // Read the hashed username and password of the user we are looking for
    let expected_username_hash = sp1_zkvm::io::read::<String>();
    let expected_password_hash = sp1_zkvm::io::read::<String>();

    // Read the hashed username and password of the user we are testing against
    let test_username_hash = sp1_zkvm::io::read::<String>();
    let test_password_hash = sp1_zkvm::io::read::<String>();

    // Read the company name the user belongs to
    let company_name = sp1_zkvm::io::read::<String>();

    // Check if the provided credentials match the expected ones
    let credentials_match = (test_username_hash == expected_username_hash) && (test_password_hash == expected_password_hash);

    // Create the output string
    let output = format!("{{\"match\": {}, \"company\": \"{}\"}}", credentials_match, company_name);

    // Write the result
    sp1_zkvm::io::write(&output);
}