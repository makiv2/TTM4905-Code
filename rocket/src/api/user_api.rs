use rocket::http::Status;
use rocket::response::status::{BadRequest, Created, NoContent};
use rocket::serde::json::Json;
use crate::models::{NewUser, User};
use crate::repository::UserRepository;
use crate::services::UserService;

// CRUD Api Endpoints
// Create
#[post("/user", format = "json", data = "<user>")]
pub(crate) async fn create_user(user: Json<NewUser>) -> crate::api::Result<Created<Json<User>>, BadRequest<String>> {

    let mut user_service = UserService::new(UserRepository::new());

    let user_result = user_service.post_user(user).await;

    Ok(Created::new("/").body(Json(user_result)))
}


// Read
#[get("/users")]
pub(crate) async fn get_users() -> crate::api::Result<Json<Vec<User>>, Status> {
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
pub(crate) async fn get_user(id: i32) -> crate::api::Result<Json<User>, Status> {
    
    let mut user_service = UserService::new(UserRepository::new());

    let user = user_service.get_user(id).await;
    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}


#[delete("/users/<id>")]
pub async fn delete_user(id: i32) -> crate::api::Result<NoContent, &'static str> {
    let mut user_service = UserService::new(UserRepository::new());

    let result = user_service.delete_user(id).await;
    match result {
        Ok(_) => Ok(NoContent),
        Err(_) => Err("Failed to delete user"),
    }
}
