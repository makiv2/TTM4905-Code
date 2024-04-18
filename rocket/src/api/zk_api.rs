use rocket::http::Status;
use rocket::response::status::{BadRequest, Created};
use rocket::serde::json::Json;
use rocket::Data;
use tokio::fs::File;
use std::fs;
use rocket::tokio::io::AsyncWriteExt;

use crate::models::{NewProof, Proof, ProofQueryResult};
use crate::repository::ZkRepository;
use crate::services::ZkService;


// CRUD Api Endpoints
// Create
#[post("/generate", format = "json", data = "<proof>")]
pub(crate) async fn generate_proof(proof: Json<NewProof>) -> crate::api::Result<Created<Json<Proof>>, BadRequest<String>> {
    
    let mut zk_service = ZkService::new(ZkRepository::new());
    
    let proof_result = zk_service.generate_proof(proof).await;
    
    Ok(Created::new("/").body(Json(proof_result)))
}


// Read
#[get("/proofs")]
pub(crate) async fn get_proofs() -> crate::api::Result<Json<Vec<ProofQueryResult>>, Status>{
    
    let mut zk_service = ZkService::new(ZkRepository::new());
    
    let proofs = zk_service.get_proofs().await;
    
    match proofs {
        Ok(proofs) => Ok(Json(proofs)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/proofs/<id>")]
pub(crate) async fn get_proof(id: i32) -> crate::api::Result<Json<ProofQueryResult>, Status> {
    
    let mut zk_service = ZkService::new(ZkRepository::new());
    
    let proof = zk_service.get_proof(id).await;
    
    match proof {
        Ok(proof) => Ok(Json(proof)),
        Err(_) => Err(Status::InternalServerError),
    }
}

// Verify (Hva gjør vi egt her?)
#[get("/verify")]
pub(crate) fn verify() -> &'static str {
    "Hello, world!"
}

// Upload JSON proof

#[post("/upload/<id>", data = "<data>")]
pub (crate) async fn upload(id: i32, data: Data<'_>) -> Result<String, std::io::Error> {
    let file_path = format!("uploads/proof_{}.json", id);
    let mut file = File::create(&file_path).await?;
    let bytes = data.open(rocket::data::ToByteUnit::megabytes(10)).into_bytes().await?;
    file.write_all(&bytes).await?;
    Ok(file_path)
}