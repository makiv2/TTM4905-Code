mod zk_service;
mod user_service;

use crate::repository::{UserRepository, ZkRepository};


pub struct UserService {
    user_repository: UserRepository,
}

pub struct ZkService {
    zk_repository: ZkRepository,
}
