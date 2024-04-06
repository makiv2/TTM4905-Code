use rocket::serde::json::Json;
use crate::models::{NewProof, Proof};
use crate::repository::ZkRepository;
use crate::services::ZkService;

impl ZkService {
    pub fn new(zk_repository: ZkRepository) -> Self {
        ZkService { zk_repository }
    }

    pub async fn generate_proof(&mut self, new_user: Json<NewUser>) -> User {
        // Her kan vi har mer logic før vi sender til repository (database) 
        let user = User {
            id: 1,
            username: new_user.username.to_string(),
            password: new_user.password.to_string(),
            message: new_user.message.to_string(),
        };

        let created_user = self.user_repository.create_user(user).await;

        return created_user;
    }

    pub async fn get_proofs(&mut self) -> Result<Vec<Proof>, &'static str> {
        // Her kan vi har mer logic før vi sender til repository (database) 

        return self.zk_repository.get_proofs().await
    }

    pub async fn get_user(&mut self, id: i32) -> Result<User, &'static str> {
        // Her kan vi har mer logic før vi sender til repository (database) 

        return self.zk_repository.get_user(id).await
    }

    // pub async fn update_user(&mut self, id: i32, updated_user: User) -> Result<User, &'static str> {
    //     // Her kan vi har mer logic før vi sender til repository (database) 
    //     
    //     return self.user_repository.update_user(id, updated_user).await
    // }

    pub async fn delete_user(&mut self, id: i32) -> Result<(), &'static str> {
        // Her kan vi har mer logic før vi sender til repository (database) 

        return self.user_repository.delete_user(id).await
    }
}
