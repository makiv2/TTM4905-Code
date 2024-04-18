use crate::models::ProofQueryResult;
use crate::models::Proof;
use crate::repository::{establish_connection_pg, ZkRepository};
use crate::schema::proofs::dsl::proofs;
use diesel::prelude::*;


impl ZkRepository {
    pub fn new() -> Self {
        let connection = establish_connection_pg();
        ZkRepository{ connection }
    }

    pub async fn save_proof(&mut self, proof: Proof) -> Proof {
        diesel::insert_into(proofs)
            .values(&proof)
            .execute(&mut self.connection)
            .expect("Error saving proof");
        proof 
    }

    pub async fn get_proofs(&mut self) -> Result<Vec<ProofQueryResult>, &'static str> {

        let results = proofs
            .limit(50)
            .load::<ProofQueryResult>(&mut self.connection)
            .expect("Error loading proofs");

        return Ok(results);
    }

    pub async fn get_proof(&mut self, proof_id: i32) -> Result<ProofQueryResult, &'static str> {
        let result = proofs.find(proof_id).first::<ProofQueryResult>(&mut self.connection);

        match result {
            Ok(proof) => Ok(proof),
            Err(_) => Err("Proof not found"),
        }
    }
}
