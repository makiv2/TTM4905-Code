// Write get and post routes for the user model

use rocket::http::Status;
// Imports
use crate::models::NewUser;
use crate::models::User;

use crate::services::UserService;

use rocket::serde::json::Json;
use rocket::response::{Debug, status::{Created, BadRequest}};
use crate::repository::UserRepository;

// Types
type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

// CRUD Api Endpoints
// Create
#[post("/user", format = "json", data = "<user>")]
pub async fn create_user(user: Json<NewUser>) -> Result<Created<Json<User>>, BadRequest<String>> {
    
    // Quickfix for now; should change this to a better approach.
    let mut user_service = UserService::new(UserRepository::new());

    let user_result = user_service.post_user(user).await;
    
    Ok(Created::new("/").body(Json(user_result)))
}


// Read
#[get("/users")]
pub async fn get_users() -> Result<Json<Vec<User>>, Status> {
    // Quickfix for now; should change this to a better approach.
    let mut user_service = UserService::new(UserRepository::new());

    let users = user_service.get_users().await;
    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}

//#[get("/users/<id>")]
//pub async fn get_user(id: i32, user_service: UserService) -> Result<Json<User>> {
//    let user = user_service.get_user(id).await;
//    match user {
//        Ok(user) => Ok(Json(user)),
//    }
//}
//
//
//// Update
//#[put("/users/<id>", format = "json", data = "<updated_user>")]
//pub async fn update_user(id: i32, updated_user: Json<User>, user_service: UserService) -> Result<Json<User>> {
//    let user = user_service.update_user(id, updated_user.into_inner()).await;
//    match user {
//        Ok(user) => Ok(Json(user)),
//    }
//}
//
//
//// Delete
//#[delete("/users/<id>")]
//pub async fn delete_user(id: i32, user_service: UserService) -> Result<NoContent, &'static str> {
//    let result = user_service.delete_user(id).await;
//    match result {
//        Ok(_) => Ok(NoContent),
//        Err(_) => Err("Failed to delete user"),
//    }
//}
//