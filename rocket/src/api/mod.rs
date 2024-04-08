pub mod user_api;
pub mod zk_api;

use rocket::http::Status;
use crate::models::NewUser;
use crate::models::User;
use crate::services::UserService;
use crate::repository::UserRepository;

use rocket::serde::json::Json;
use rocket::response::{Debug, status::{Created, BadRequest}};
use rocket::response::status::NoContent;

// Types
type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;


// CRUD Api Endpoints
// Create
#[post("/user", format = "json", data = "<user>")]
pub(crate) async fn create_user(user: Json<NewUser>) -> Result<Created<Json<User>>, BadRequest<String>> {

    // Quickfix for now; should change this to a better approach.
    let mut user_service = UserService::new(UserRepository::new());

    let user_result = user_service.post_user(user).await;

    Ok(Created::new("/").body(Json(user_result)))
}


// Read
#[get("/users")]
pub(crate) async fn get_users() -> Result<Json<Vec<User>>, Status> {
    // Quickfix for now; should change this to a better approach.
    let mut user_service = UserService::new(UserRepository::new());

    let users = user_service.get_users().await;
    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}

// Read
#[get("/users/<id>")]
pub(crate) async fn get_user(id: i32) -> Result<Json<User>, Status> {
    let mut user_service = UserService::new(UserRepository::new());

    let user = user_service.get_user(id).await;
    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}


#[delete("/users/<id>")]
pub async fn delete_user(id: i32) -> Result<NoContent, &'static str> {
    let mut user_service = UserService::new(UserRepository::new());

    let result = user_service.delete_user(id).await;
    match result {
        Ok(_) => Ok(NoContent),
        Err(_) => Err("Failed to delete user"),
    }
}
