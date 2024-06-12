use rocket::serde::json::Json;

use crate::models::{CredentialsMessage, Proof};
use crate::repository::ZkRepository;
use crate::services::ZkService;


// CRUD Api Endpoints
// Create
#[post("/generate_proof", format = "json", data = "<credentials_with_message>")]
pub(crate) async fn generate_proof(credentials_with_message: Json<CredentialsMessage>) -> String {

    let mut zk_service = ZkService::new(ZkRepository::new()); // Initialize the service and the repository

    // Get the data for the API call
    let messageb64 = &credentials_with_message.messageb64;
    let signatureb64 = &credentials_with_message.signatureb64;
    let companyb64 = &credentials_with_message.companyb64;

    let proof_result = zk_service.generate_proof(messageb64, signatureb64, companyb64).await;

    return proof_result;
}


// Read
#[get("/proofs")]
pub(crate) async fn get_proofs() -> String {

    let mut zk_service = ZkService::new(ZkRepository::new());
    
    let proofs = zk_service.get_proofs().await;

    return proofs;
}

#[get("/proofs/<id>")]
pub(crate) async fn get_proof(id: i32) -> String {
    
    let mut zk_service = ZkService::new(ZkRepository::new());
    
    let proof_json_string = zk_service.get_proof(id).await;

    return proof_json_string;
}

#[get("/proofs/<id>/raw")]
pub(crate) async fn get_raw_proof(id: i32) -> Json<Proof> {
    
    let mut zk_service = ZkService::new(ZkRepository::new());
    
    let raw_proof_json_string = zk_service.get_raw_proof(id).await;

    return raw_proof_json_string;
}

// Verify TODO
#[get("/verify")]
pub(crate) fn verify() -> &'static str {
    "Hello, world!"
}
