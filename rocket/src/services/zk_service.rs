use tokio::process::Command;
use crate::models::{Proof};
use crate::repository::ZkRepository;
use crate::services::ZkService;

use std::fs::File;
use std::io::Read;


impl ZkService {
    pub fn new(zk_repository: ZkRepository) -> Self {
        ZkService { zk_repository }
    }

    pub async fn generate_proof(&mut self, username: &String, password: &String, company: &String, message: &String) -> String { //TODO Rewrite return type to proof

        // Run the script executable
        let _output = Command::new("/sp1/auxiliary/target/release/auxiliary")
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
        let proof = Self::read_json();
        let db_proof = proof.to_db_proof();
        self.zk_repository.save_proof(db_proof).await;
        return String::from("Failure") 
    }

    pub async fn get_proofs(&mut self) -> String {
        // Her kan vi har mer logic før vi sender til repository (database) 
        let proofs = self.zk_repository.get_proofs().await;
        
        // Convert all the proofs in proofs to json and return it
        for db_proof in proofs {
            let proof = db_proof.to_proof();
            println!("{:?}", proof);
        }
        
        return String::from("PROOFS ABOVE------------------------------------------")
    }

    pub async fn get_proof(&mut self, id: i32) -> String {
        // Her kan vi har mer logic før vi sender til repository (database) 
        //return self.zk_repository.get_proof(id).await
        return String::from("Failure") 
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
}
