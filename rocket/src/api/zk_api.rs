use rocket::http::Status;
use rocket::serde::json::Json;

use crate::models::{CredentialsMessage, ProofQueryResult};
use crate::repository::ZkRepository;
use crate::services::ZkService;


// CRUD Api Endpoints
// Create
#[post("/generate_proof", format = "json", data = "<credentials_with_message>")]
pub(crate) async fn generate_proof(credentials_with_message: Json<CredentialsMessage>) -> String {

    let mut zk_service = ZkService::new(ZkRepository::new()); // Initialize the service and the repository
    
    // Get the data for the API call
    let username = &credentials_with_message.username;
    let password = &credentials_with_message.password;
    let company = &credentials_with_message.company;
    let message = &credentials_with_message.message;
    
    let proof_result = zk_service.generate_proof(username, password, company, message).await;
    
    return proof_result;
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
