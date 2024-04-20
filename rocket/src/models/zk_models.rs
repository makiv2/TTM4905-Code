use super::super::schema::proofs; 

use diesel::{prelude::*};
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = proofs)]
pub struct DBProof {
    pub(crate) id: i32,
    pub(crate) proof: String,
    pub(crate) stdout_buffer_data: Vec<u8>, 
}

#[derive(Queryable, Serialize)]
pub struct ProofQueryResult {
    pub id: i32,
    pub proof: String,
    pub company: String,
    pub message: String,
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
    pub(crate) company: String,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) message: String,
}


// Implement a conversion function from Proof to DBProof
impl Proof {
    pub(crate) fn to_db_proof(&self) -> DBProof {
        DBProof {
            id: 0, 
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
