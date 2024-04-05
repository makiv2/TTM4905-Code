use crate::models::Proof;
use crate::repository::{establish_connection_pg, ZkRepository};
use crate::schema::proofs::dsl::proofs;
use diesel::prelude::*;


impl ZkRepository {
    pub fn new() -> Self {
        let connection = establish_connection_pg();
        ZkRepository{ connection }
    }

    pub async fn store_proof(&mut self, proof: Proof) -> Proof {
        diesel::insert_into(proofs)
            .values(&proof)
            .execute(&mut self.connection)
            .expect("Error saving proof");
        proof 
    }

    pub async fn get_proofs(&mut self) -> Result<Vec<Proof>, &'static str> {

        let results = proofs
            .limit(50)
            .load::<Proof>(&mut self.connection)
            .expect("Error loading proofs");

        return Ok(results);
    }

    pub async fn get_proof(&mut self, proof_id: i32) -> Result<Proof, &'static str> {
        let result = proofs.find(proof_id).first::<Proof>(&mut self.connection);

        match result {
            Ok(proof) => Ok(proof),
            Err(_) => Err("Proof not found"),
        }
    }
}
