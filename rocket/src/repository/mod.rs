mod user_repository;
mod zk_repository;


use std::env;
use diesel::prelude::*;
use dotenvy::dotenv;


pub struct UserRepository {
    connection: PgConnection,
}

pub struct ZkRepository {
    connection: PgConnection,
}

fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
