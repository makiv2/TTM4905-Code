extern crate diesel;
extern crate rocket;

use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post};
use crate::models;
use crate::models::NewUser;
use crate::schema;
use std::env;

pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

// TODO: Move first part to api/mod.rs
#[post("/user", format = "json", data = "<user>")]
pub fn create_user(user: Json<NewUser>) -> Result<Created<Json<NewUser>>> {
    use models::User;
    let connection = &mut establish_connection_pg();

    let new_user = User {
        id: 1,
        username: user.username.to_string(),
        password: user.password.to_string(),
        message: user.message.to_string(),
    };

    diesel::insert_into(self::schema::users::dsl::users)
        .values(&new_user)
        .execute(connection)
        .expect("Error saving new post");
    Ok(Created::new("/").body(user))
}

#[get("/users")]
pub fn get_users() -> Result<Json<Vec<models::User>>> {
    use schema::users::dsl::*;
    let connection = &mut establish_connection_pg();
    let results = users
        .limit(5)
        .load::<models::User>(connection)
        .expect("Error loading posts");
    Ok(Json(results))
}
