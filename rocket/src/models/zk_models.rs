use super::super::schema::proofs;

use diesel::{prelude::*};
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = proofs)]
pub struct Proof {
    pub(crate) id: i32,
    pub(crate) proof: String,
    pub(crate) company: String, 
    pub(crate) message: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewProof {
    pub(crate) proof: String,
    pub(crate) company: String,
    pub(crate) message: String,
}
