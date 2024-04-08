use rocket::http::Status;
use rocket::serde::json::Json;


// CRUD Api Endpoints
// Create
#[post("/generate", format = "json", data = "<proof>")]
pub(crate) async fn create_proof(proof: Json<String>) -> std::result::Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from rust!")))
}

// Read
#[get("/proofs")]
pub(crate) fn get_proofs() -> &'static str {
    "Hello, world!"
}

#[get("/proofs/<id>")]
pub(crate) fn get_proof_by_id(id: i32) -> &'static str {
    "Hello, world!"
}

// Verify
#[get("/verify")]
pub(crate) fn verify() -> &'static str {
    "Hello, world!"
}
