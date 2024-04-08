use crate::models::User;
use crate::repository::{establish_connection_pg, UserRepository};
use crate::schema::users::dsl::users;
use diesel::prelude::*;


impl UserRepository {
    pub fn new() -> Self {
        let connection = establish_connection_pg();
        UserRepository { connection }
    }

    pub async fn create_user(&mut self, user: User) -> User {
        diesel::insert_into(users)
            .values(&user)
            .execute(&mut self.connection)
            .expect("Error saving new user");
        user
    }

    pub async fn get_users(&mut self) -> Result<Vec<User>, &'static str> {

        let results = users
            .limit(5)
            .load::<User>(&mut self.connection)
            .expect("Error loading users");

        return Ok(results);
    }

    pub async fn get_user(&mut self, user_id: i32) -> Result<User, &'static str> {
        let result = users.find(user_id).first::<User>(&mut self.connection);

        match result {
            Ok(user) => Ok(user),
            Err(_) => Err("User not found"),
        }
    }

    // pub async fn update_user(&mut self, other_id: i32, updated_user: User) -> Result<User, &'static str> {
    //     let result = diesel::update(users.find(id))
    //         .set(&updated_user)
    //         .get_result(&mut self.connection);

    //     match result {
    //         Ok(user) => Ok(user),
    //         Err(_) => Err("Failed to update user"),
    //     }
    // }

    pub async fn delete_user(&mut self, user_id: i32) -> Result<(), &'static str> {
        let result = diesel::delete(users.find(user_id)).execute(&mut self.connection);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to delete user"),
        }
    }
}