#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // Predefined credentials
    // let expected_username_hash = 123456789u128; // Example hash of "user123"
    // let expected_password_hash = 987654321u128; // Example hash of "pass123"

    // Dummy username test
    let expected_username: String = "red".to_string();
    let expected_password: String = "bull".to_string();

    // Read username and password hashes from input
    let username_hash = sp1_zkvm::io::read::<String>();
    let password_hash = sp1_zkvm::io::read::<String>();

    // Check if the provided credentials match the expected ones
    let credentials_match = (username_hash == expected_username) && (password_hash == expected_password);

    // Write the result
    sp1_zkvm::io::write(&credentials_match);
}
