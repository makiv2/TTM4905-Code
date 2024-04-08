pub mod user_api;
pub mod zk_api;

use rocket::response::{Debug};

// Types
type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;
