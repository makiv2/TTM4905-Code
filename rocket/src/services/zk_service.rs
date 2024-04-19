use std::fs;
use rocket::serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::process::Command;
use crate::models::{ ProofQueryResult };
use crate::repository::ZkRepository;
use crate::services::ZkService;

use std::fs::File;
use std::io::Read;
use rocket::serde::json::Json;

#[derive(Debug, Deserialize, Serialize)]
struct MyData {
    proof: String,
    stdout: Stdout,
}

#[derive(Debug, Deserialize, Serialize)]
struct Stdout {
    buffer: Buffer,
}

#[derive(Debug, Deserialize, Serialize)]
struct Buffer {
    data: Vec<u8>,
}



impl ZkService {
    pub fn new(zk_repository: ZkRepository) -> Self {
        ZkService { zk_repository }
    }

    pub async fn generate_proof(&mut self, username: &String, password: &String, company: &String, message: &String) -> String { //TODO Rewrite return type to proof

        // Run the script executable
        let output = Command::new("/sp1/auxiliary/target/release/auxiliary")
            .arg(username)
            .arg(password)
            .arg(company)
            .arg(message)
            .output()
            .await
            .expect("failed to execute script");

        // Ignore if it was successful now
        // Take care of that later
        // Take the file proof.json and make it a db object and save it in the db
        let _json = Self::read_json().expect("Failed to read JSON file");


        // Check if the execution was successful
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("This is the stdout {}", stdout);

        // Check if the execution was successful
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("This is the stderr {}", stderr);

        if output.status.success() {
            // Get the proof content from the script output
            let proof_content = String::from_utf8_lossy(&output.stdout);

            // Check if the output is equal to "No matching user found in the database."
            if proof_content.trim() == "No matching user found in the database." {
            } else {
                // Read json file created
                let json_file_path = "credentials_check_proof.json";
                let json_content = fs::read_to_string(json_file_path).expect("Failed to read JSON file");

                // Parse the JSON content
                let json: Value = serde_json::from_str(&json_content).expect("Failed to parse JSON");

                // Extract the proof from the JSON
                let proof = json["proof"].as_str().expect("Proof not found in JSON").to_string();

                // Extract the output from the JSON
                let output_data = json["stdout"]["buffer"]["data"].as_array().expect("Output data not found in JSON");
                let mut output_string = String::from_utf8(output_data.iter().map(|v| v.as_u64().unwrap() as u8).collect()).expect("Failed to convert output to string");
                output_string.remove(0).to_string();
                output_string.trim().to_string();

                // The output string is malformed json so we won't use it further but it should be possible
                println!("{:?}", output_string);

                // Parse the output string
                println!("Do I fail here?");
                println!("Output String: {}", output_string);
                return proof
            }
            return String::from("Failure") //TODO Rewrite return type to proof;
        }
        // let created_proof = self.zk_repository.save_proof(proof).await;

        return String::from("Failure") //TODO Rewrite return type to proof;
    }

    pub async fn get_proofs(&mut self) -> Result<Vec<ProofQueryResult>, &'static str> {
        // Her kan vi har mer logic før vi sender til repository (database) 

        return self.zk_repository.get_proofs().await
    }

    pub async fn get_proof(&mut self, id: i32) -> Result<ProofQueryResult, &'static str> {
        // Her kan vi har mer logic før vi sender til repository (database) 

        return self.zk_repository.get_proof(id).await
    }

    // pub async fn update_user(&mut self, id: i32, updated_user: User) -> Result<User, &'static str> {
    //     // Her kan vi har mer logic før vi sender til repository (database) 
    //     
    //     return self.user_repository.update_user(id, updated_user).await
    // }

    fn read_json() -> Result<Json<MyData>, Box<dyn std::error::Error>> {
        // Open the JSON file
        let mut file = File::open("credentials_check_proof.json")?;

        // Read the file content into a string
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Parse the JSON string into a JSON object
        let json_data: MyData = serde_json::from_str(&contents)?;

        // Here, you can send `json_data` to your database
        // For demonstration, let's just return the parsed JSON data
        print!("{:?}", json_data);
        print!("Read the data?");
        Ok(Json(json_data))
    }
}
