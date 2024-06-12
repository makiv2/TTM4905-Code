use tokio::process::Command;
use crate::models::Proof;
use crate::repository::ZkRepository;
use crate::services::ZkService;
use serde_json::json;
use serde_json::Value;

use std::fs::File;
use std::io::Read;
use rocket::serde::json::Json;


impl ZkService {
    pub fn new(zk_repository: ZkRepository) -> Self {
        ZkService { zk_repository }
    }

    pub async fn generate_proof(&mut self, messageb64: &String, signatureb64: &String, companyb64: &String) -> String { //TODO Rewrite return type to proof

        // Run the script executable
        let _output = Command::new("../sp1/auxiliary/target/release/auxiliary")
            .arg(messageb64)
            .arg(signatureb64)
            .arg(companyb64)
            .output()
            .await
            .expect("failed to execute script");

        // Ignore if it was successful now
        // Take care of that later
        // Take the file proof.json and make it a db object and save it in the db
        // Check if output is equal to "No matching user found in the database."
        let output_string = String::from_utf8_lossy(&_output.stdout);
        println!("Output: {}", output_string);
        if output_string.trim() == "No compatible signature found in the database." {
            return String::from("Failure.")
        }
        let proof = Self::read_json();
        let db_proof = proof.to_db_proof();
        self.zk_repository.save_proof(db_proof).await;
        return String::from("Success, proof saved.") 
    }

    pub async fn get_proofs(&mut self) -> String {
        // Her kan vi har mer logic før vi sender til repository (database) 
        let proofs = self.zk_repository.get_proofs().await;
        println!("Proofs length: {}", proofs.len());
        // Convert all the proofs in proofs to json and return it
        // Save all proofs
        let mut json_array: Vec<Value> = Vec::new();
        for db_proof in proofs {
            println!("Proof: {:?}", db_proof.id);

            let proof = db_proof.to_proof();
            
            let proof_id = db_proof.id;
            let output_data = proof.stdout.buffer.data;
            print!("Proof: {:?}", db_proof.id);
            println!("Output data: {:?}", output_data);

            match Self::process_output(output_data, proof_id) {
                Ok(json_value) => {
                    json_array.push(json_value);
                },
                Err(error) => {
                    println!("Error processing output: {}", error);
                }
            }
        }
        
        if json_array.is_empty() {
            String::from("No proofs found")
        } else {
            let json_string = serde_json::to_string(&json_array).unwrap();
            json_string
        }
    }

    pub async fn get_proof(&mut self, id: i32) -> String {

        let db_proof = self.zk_repository.get_proof(id).await;

        let proof = db_proof.to_proof();

        let proof_id = db_proof.id;
        let output_data = proof.stdout.buffer.data;
        print!("Proof: {:?}", db_proof.id);
        println!("Output data: {:?}", output_data);

        match Self::process_output(output_data, proof_id) {
            Ok(json_value) => {
                let json_string = json_value.to_string();
                return json_string;
            },
            Err(error) => {
                println!("Error processing output: {}", error);
            }
        }
        return String::from("Failure")
    }

    pub async fn get_raw_proof(&mut self, id: i32) -> Json<Proof> {

        let db_proof = self.zk_repository.get_proof(id).await;
        let proof = db_proof.to_proof();
        
        Json(proof)
    }

    // Private helper function 
    fn read_json() -> Proof {
        // Open the JSON file
        let mut file = File::open("credentials_check_proof.json").unwrap();

        // Read the file content into a string
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Parse the JSON file into a proof object
        let proof: Proof = serde_json::from_str(&contents).unwrap();

        return proof;
    }

    fn process_output(output_data: Vec<u8>, proof_id: i32) -> Result<Value, String> {
        // Process the output from the script
        return if let Ok(input_str) = String::from_utf8(output_data) {
            if let Some(start_index) = input_str.find('{') {
                if let Some(json_str) = input_str.get(start_index..) {
                    println!("Cleaned JSON: {}", json_str);
                    let mut json_value: Value = serde_json::from_str(json_str).unwrap();
                    json_value["id"] = json!(proof_id);
                    println!("JSON value: {}", json_value.to_string());
                    Ok(json_value)
                } else {
                    println!("Invalid JSON format");
                    Err(String::from("Invalid JSON format"))
                }
            } else {
                println!("No JSON object found in the input");
                Err(String::from("No JSON object found in the input"))
            }
        } else {
            println!("Failed to convert input bytes to string");
            Err(String::from("Failed to convert input bytes to string"))
        }
    }
}
