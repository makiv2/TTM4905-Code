// In your services/mod.rs file:

use crate::repository::UserRepository;
use crate::models::{User, NewUser};

use rocket::serde::json::Json;

pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn new(user_repository: UserRepository) -> Self {
        UserService { user_repository }
    }

    pub async fn post_user(&self, user: Json<NewUser>) -> Result<User, &'static str> {
        // Her kan vi har mer logic før vi sender til repository (database) 
        let new_user = User {
            id: 1,
            username: user.username.to_string(),
            password: user.password.to_string(),
            message: user.message.to_string(),
        };
        
        return self.user_repository.create_user(new_user).await
    }

    pub async fn get_users(&self) -> Result<Vec<User>, &'static str> {
        // Her kan vi har mer logic før vi sender til repository (database) 
        
        return self.user_repository.get_users().await
    }

    pub async fn get_user(&self, id: i32) -> Result<User, &'static str> {
        // Her kan vi har mer logic før vi sender til repository (database) 
        
        return self.user_repository.get_user(id).await
    }

    pub async fn update_user(&self, id: i32, updated_user: User) -> Result<User, &'static str> {
        // Her kan vi har mer logic før vi sender til repository (database) 
        
        return self.user_repository.update_user(id, updated_user).await
    }

    pub async fn delete_user(&self, id: i32) -> Result<(), &'static str> {
        // Her kan vi har mer logic før vi sender til repository (database) 
        
        return self.user_repository.delete_user(id).await
    }
}


