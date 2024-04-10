#! /bin/bash


# Just in case to make sure the rocket backend does not mess with sp1
cd /sp1

# Install sp1
curl -sSL https://sp1.succinct.xyz | bash
source /root/.bashrc
sp1up
cargo prove --version

# Go back into the rocket backend (app)
cd /app

diesel migration run

cargo run --release
