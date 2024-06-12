use super::super::schema::proofs; 

use diesel::{prelude::*};
use serde::{Serialize, Deserialize};
use rand::Rng;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = proofs)]
pub struct DBProof {
    pub(crate) id: i32,
    pub(crate) proof: String,
    pub(crate) stdout_buffer_data: Vec<u8>, 
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Proof {
    pub(crate) proof: String,
    pub(crate) stdout: Stdout,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stdout {
    pub(crate) buffer: Buffer,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Buffer {
    pub(crate) data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct CredentialsMessage {
    pub(crate) companyb64: String,
    pub(crate) signatureb64: String,
    pub(crate) messageb64: String,
}

// Implement a conversion function from Proof to DBProof
impl Proof {
    pub(crate) fn to_db_proof(&self) -> DBProof {
        println!("SAVING PROOF");
        let random_id = rand::thread_rng().gen_range(1..99999);
        DBProof {
            id: random_id, 
            proof: self.proof.clone(),
            stdout_buffer_data: self.stdout.buffer.data.clone(), 
        }
    }
}

// Implement a conversion function from DBProof to Proof
impl DBProof {
    pub(crate) fn to_proof(&self) -> Proof {
        Proof {
            proof: self.proof.clone(),
            stdout: Stdout {
                buffer: Buffer {
                    data: self.stdout_buffer_data.clone(),
                }
            }
        }
    }
}
