use super::super::schema::proofs;

use diesel::{prelude::*};
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = proofs)]
pub struct Proof {
    pub(crate) id: Option<i32>,
    pub(crate) proof: String,
    pub(crate) company: String, 
    pub(crate) message: String,
}

#[derive(Queryable, Serialize)]
pub struct ProofQueryResult {
    pub id: i32,
    pub proof: String,
    pub company: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewProof {
    pub(crate) proof: String,
    pub(crate) company: String,
    pub(crate) message: String,
}

#[derive(Serialize, Deserialize)]
pub struct CredentialsMessage {
    pub(crate) company: String,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) message: String,
}
