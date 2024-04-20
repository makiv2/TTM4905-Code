use crate::models::DBProof;
use crate::repository::{establish_connection_pg, ZkRepository};
use crate::schema::proofs::dsl::proofs;
use diesel::prelude::*;


impl ZkRepository {
    pub fn new() -> Self {
        let connection = establish_connection_pg();
        ZkRepository{ connection }
    }

    pub async fn save_proof(&mut self, proof: DBProof) -> String {

        diesel::insert_into(proofs)
            .values(&proof)
            .execute(&mut self.connection)
            .expect("Error saving proof");
        String::from("Proof saved")
    }

    pub async fn get_proofs(&mut self) -> Vec<DBProof> {

        let results = proofs
            .limit(50)
            .load::<DBProof>(&mut self.connection)
            .expect("Error loading proofs");

        return results;
    }

    //pub async fn get_proof(&mut self, proof_id: i32) -> Result<ProofQueryResult, &'static str> {
    //    let result = proofs.find(proof_id).first::<ProofQueryResult>(&mut self.connection);

    //    match result {
    //        Ok(proof) => Ok(proof),
    //        Err(_) => Err("Proof not found"),
    //    }
    //}
}
