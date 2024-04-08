pub mod user_api;
pub mod zk_api;

use rocket::response::{Debug};

// Types
type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;


// Health check that just simply returns 200 OK on a get request
#[get("/health")]
pub fn health() -> &'static str {
    "200 OK"
}
