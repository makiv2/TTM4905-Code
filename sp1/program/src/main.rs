#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // Predefined credentials
    let expected_username_hash = 123456789u128; // Example hash of "user123"
    let expected_password_hash = 987654321u128; // Example hash of "pass123"

    // Read username and password hashes from input
    let username_hash = sp1_zkvm::io::read::<u128>();
    let password_hash = sp1_zkvm::io::read::<u128>();

    // Check if the provided credentials match the expected ones
    let credentials_match = (username_hash == expected_username_hash) && (password_hash == expected_password_hash);

    // Write the result
    sp1_zkvm::io::write(&credentials_match);
}
